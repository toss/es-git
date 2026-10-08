import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { describe, expect, it, vi } from 'vitest';
import { type CredentialCallbackArgs, cloneRepository, initRepository, openRepository } from '../index';
import { useFixture } from './fixtures';
import { basicAuthorization, createGitHttpServer, createRejectingServer, hasGitHttpBackend } from './http-server';
import { makeTmpDir } from './tmp';

const auth = { username: 'alice', password: 'secret' };

async function serveBareRepository() {
  const root = await makeTmpDir('http-root');
  const source = await openRepository(await useFixture('commits'));
  await cloneRepository(source.path(), path.join(root, 'repo.git'), { bare: true });
  const head = source.head().target()!;
  const server = await createGitHttpServer(root, auth);
  return { root, head, server, url: `${server.url}/repo.git` };
}

describe('credential callback', () => {
  it('passes the url, the username in the url and the allowed types', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn((_args: CredentialCallbackArgs) => null);

    await expect(
      cloneRepository(`${server.url.replace('http://', 'http://bob@')}/repo.git`, await makeTmpDir('clone'), {
        fetch: { credential },
      })
    ).rejects.toThrow(/credential callback returned no credential/);
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback returned no credential/);

    expect(credential).toHaveBeenCalledTimes(2);
    expect(credential.mock.calls[0]![0]).toEqual({
      url: `${server.url.replace('http://', 'http://bob@')}/repo.git`,
      usernameFromUrl: 'bob',
      allowedTypes: expect.arrayContaining(['Plain']),
    });
    expect(credential.mock.calls[1]![0]).toEqual({
      url: `${server.url}/repo.git`,
      usernameFromUrl: null,
      allowedTypes: expect.arrayContaining(['Plain']),
    });
  });

  it('aborts when the callback returns undefined', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn(() => undefined);
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback returned no credential for http:\/\/127\.0\.0\.1:\d+\/repo\.git/);
    expect(credential).toHaveBeenCalledTimes(1);
  });

  it('aborts with the reason when the callback throws', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn(() => {
      throw new Error('no token for this host');
    });
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback threw: Error: no token for this host/);
    expect(credential).toHaveBeenCalledTimes(1);
  });

  it('aborts with the reason when the returned promise rejects', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn(async () => {
      throw new Error('keychain is locked');
    });
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback rejected: Error: keychain is locked/);
    expect(credential).toHaveBeenCalledTimes(1);
  });

  it('aborts when the promise resolves to null', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn(async () => null);
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback returned no credential/);
    expect(credential).toHaveBeenCalledTimes(1);
  });

  it('aborts when the callback returns a credential type the remote does not accept', async () => {
    const server = await createRejectingServer();
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: { credential: () => ({ type: 'SSHKeyFromAgent' }) },
      })
    ).rejects.toThrow(/credential callback returned a SSHKeyFromAgent credential, but .* accepts only .*Plain/);
  });

  it('aborts when the callback returns a credential without its required field', async () => {
    const server = await createRejectingServer();
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: { credential: () => ({ type: 'Plain' }) as any },
      })
    ).rejects.toThrow(/credential callback returned a Plain credential without password/);
  });

  it('aborts with the reason when the callback throws a non-error value', async () => {
    const server = await createRejectingServer();
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: {
          credential: () => {
            throw 'no token';
          },
        },
      })
    ).rejects.toThrow(/credential callback threw: no token/);
  });

  it('aborts when the thrown or rejected value cannot be turned into a string', async () => {
    const server = await createRejectingServer();
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: {
          credential: () => {
            throw Symbol('no token');
          },
        },
      })
    ).rejects.toThrow(/credential callback threw: unknown error/);
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: { credential: () => Promise.reject(Symbol('no token')) },
      })
    ).rejects.toThrow(/credential callback rejected: unknown error/);
  });

  it('aborts when reading the returned credential throws', async () => {
    const server = await createRejectingServer();
    const credential = () => ({
      get type(): 'Plain' {
        throw new Error('getter failed');
      },
      password: 'secret',
    });
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback returned an invalid credential/);
  });

  it('aborts when the callback returns an invalid credential', async () => {
    const server = await createRejectingServer();
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), {
        fetch: { credential: () => ({ type: 'Unknown' }) as any },
      })
    ).rejects.toThrow(/credential callback returned an invalid credential/);
  });

  it('gives up after 10 rejected credentials', async () => {
    const server = await createRejectingServer();
    const credential = vi.fn(() => ({ type: 'Plain' as const, username: 'alice', password: 'wrong' }));
    await expect(
      cloneRepository(`${server.url}/repo.git`, await makeTmpDir('clone'), { fetch: { credential } })
    ).rejects.toThrow(/credential callback was called 10 times .* giving up/);
    expect(credential).toHaveBeenCalledTimes(10);
    expect(server.authorizations).toContain(basicAuthorization({ username: 'alice', password: 'wrong' }));
  });

  it('is called by fetch, push, clone and submodule clone', async () => {
    const server = await createRejectingServer();
    const url = `${server.url}/repo.git`;
    const credential = vi.fn((_args: CredentialCallbackArgs) => null);

    const repo = await openRepository(await useFixture('commits'));
    const remote = repo.createRemote('origin', url);
    await expect(remote.fetch([], { fetch: { credential } })).rejects.toThrow(/returned no credential/);
    await expect(remote.push(['refs/heads/main:refs/heads/main'], { credential })).rejects.toThrow(
      /returned no credential/
    );
    await expect(cloneRepository(url, await makeTmpDir('clone'), { fetch: { credential } })).rejects.toThrow(
      /returned no credential/
    );
    const submodule = repo.submodule(url, 'sub', true);
    await expect(submodule.clone({ fetch: { credential } })).rejects.toThrow(/returned no credential/);

    expect(credential).toHaveBeenCalledTimes(4);
    expect(credential.mock.calls.every(([args]) => args.url === url)).toBe(true);
  });

  it('is called by submodule update', async () => {
    const server = await createRejectingServer();
    const url = `${server.url}/repo.git`;
    const repo = await openRepository(await useFixture('commits'));
    const source = pathToFileURL(await useFixture('commits')).toString();
    const added = repo.submodule(source, 'sub', true);
    await added.clone();
    added.addToIndex(true);
    added.addFinalize();
    repo.submoduleSetUrl('sub', url);
    // A checked-out superproject has an empty directory for a submodule that is not cloned yet.
    await fs.rm(path.join(repo.workdir()!, 'sub'), { recursive: true });
    await fs.mkdir(path.join(repo.workdir()!, 'sub'));
    await fs.rm(path.join(repo.path(), 'modules', 'sub'), { recursive: true });

    const submodule = repo.getSubmodule('sub');
    await submodule.sync();
    const credential = vi.fn((_args: CredentialCallbackArgs) => null);
    await expect(submodule.update(true, { fetch: { credential } })).rejects.toThrow(/returned no credential/);
    expect(credential).toHaveBeenCalledTimes(1);
    expect(credential.mock.calls[0]![0].url).toEqual(url);
  });

  it('is accepted but not called by prune, which does not connect', async () => {
    const remotePath = await makeTmpDir('remote-bare');
    await initRepository(remotePath, { bare: true });
    const repo = await openRepository(await useFixture('commits'));
    const remote = repo.createRemote('origin', remotePath);
    await remote.fetch([]);
    const credential = vi.fn(() => null);
    await remote.prune({ credential });
    expect(credential).not.toHaveBeenCalled();
  });
});

describe.skipIf(!hasGitHttpBackend)('credential over authenticated HTTP', () => {
  it('keeps accepting a credential object', async () => {
    const { head, url } = await serveBareRepository();
    const repo = await cloneRepository(url, await makeTmpDir('clone'), {
      fetch: { credential: { type: 'Plain', ...auth } },
    });
    expect(repo.head().target()).toEqual(head);
  });

  it('clones and fetches with a credential returned by the callback', async () => {
    const { head, server, url } = await serveBareRepository();
    const credential = vi.fn(({ usernameFromUrl }: CredentialCallbackArgs) => ({
      type: 'Plain' as const,
      username: usernameFromUrl ?? auth.username,
      password: auth.password,
    }));
    const repo = await cloneRepository(url, await makeTmpDir('clone'), { fetch: { credential } });
    expect(repo.head().target()).toEqual(head);
    expect(server.authorizations).toContain(basicAuthorization(auth));

    await repo.getRemote('origin').fetch([], { fetch: { credential } });
    expect(credential).toHaveBeenCalled();
  });

  it('waits for a credential from a promise', async () => {
    const { head, url } = await serveBareRepository();
    const credential = vi.fn(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
      return { type: 'Plain' as const, ...auth };
    });
    const repo = await cloneRepository(url, await makeTmpDir('clone'), { fetch: { credential } });
    expect(repo.head().target()).toEqual(head);
    expect(credential).toHaveBeenCalled();
  });

  it('retries with the next credential after a rejected one', async () => {
    const { head, url } = await serveBareRepository();
    const passwords = ['wrong', auth.password];
    const credential = vi.fn(() => ({ type: 'Plain' as const, username: auth.username, password: passwords.shift()! }));
    const repo = await cloneRepository(url, await makeTmpDir('clone'), { fetch: { credential } });
    expect(repo.head().target()).toEqual(head);
    expect(credential).toHaveBeenCalledTimes(2);
  });

  it('pushes with a credential returned by the callback', async () => {
    const { root, url } = await serveBareRepository();
    const repo = await openRepository(await useFixture('commits'));
    const head = repo.head().target()!;
    const remote = repo.createRemote('origin', url);
    await remote.push(['refs/heads/main:refs/heads/pushed'], {
      credential: () => ({ type: 'Plain', ...auth }),
    });
    const served = await openRepository(path.join(root, 'repo.git'));
    expect(served.getReference('refs/heads/pushed').target()).toEqual(head);
  });

  it('picks a credential per url', async () => {
    const first = await serveBareRepository();
    const secondRoot = await makeTmpDir('http-root');
    await fs.cp(path.join(first.root, 'repo.git'), path.join(secondRoot, 'repo.git'), { recursive: true });
    const secondAuth = { username: 'carol', password: 'other-secret' };
    const second = await createGitHttpServer(secondRoot, secondAuth);
    const credentials = new Map([
      [new URL(first.server.url).host, { type: 'Plain' as const, ...auth }],
      [new URL(second.url).host, { type: 'Plain' as const, ...secondAuth }],
    ]);
    const credential = ({ url }: CredentialCallbackArgs) => credentials.get(new URL(url).host) ?? null;

    const repo = await cloneRepository(first.url, await makeTmpDir('clone'), { fetch: { credential } });
    const remote = repo.createRemote('second', `${second.url}/repo.git`);
    await remote.fetch([], { fetch: { credential } });

    expect(repo.getReference('refs/remotes/second/main').target()).toEqual(first.head);
    expect(second.authorizations).toContain(basicAuthorization(secondAuth));
  });
});
