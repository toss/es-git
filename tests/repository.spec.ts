import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it, vi } from 'vitest';
import { cloneRepository, initRepository, openRepository } from '../index';
import { isTarget } from './env';
import { useFixture } from './fixtures';
import { hasGit, serveGitRepositories } from './git-server';
import { makeTmpDir } from './tmp';

describe('Repository', () => {
  it('init git repository', async () => {
    const p = await useFixture('notgit');
    await initRepository(p);
  });

  it('init git repository with options', async () => {
    const p = await useFixture('notgit');
    const repo = await initRepository(p, { bare: true, initialHead: 'my-branch' });
    expect(repo.isBare()).toBe(true);
  });

  it('open git repository', async () => {
    const p = await useFixture('empty');
    await openRepository(p);
  });

  it('open git bare repository', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(path.join(p, '.git'), { bare: true });
    expect(repo.isBare()).toBe(true);
  });

  it('open git repository in sub directory', async () => {
    const p = await useFixture('empty');
    const subdir = path.join(p, 'a', 'b', 'c');
    await fs.mkdir(subdir, { recursive: true });
    await openRepository(subdir);
    await expect(() => openRepository(subdir, { noSearch: true })).rejects.toThrowError(
      /libgit2 error: could not find repository/
    );
  });

  it('error if given path is not git repository', async () => {
    const p = await useFixture('notgit');
    await expect(openRepository(p, { noSearch: true })).rejects.toThrowError();
  });

  it('clone from local', async () => {
    const localPath = await useFixture('commits');
    const p = await makeTmpDir('clone');
    await cloneRepository(localPath, p);
    await expect(fs.readFile(path.join(p, 'first'), 'utf8')).resolves.toEqual(expect.stringContaining('first'));
  });

  it('shallow clone', { skip: !hasGit }, async () => {
    const root = await useFixture('commits');
    const server = await serveGitRepositories(root);
    try {
      const p = await makeTmpDir('clone');
      const repo = await cloneRepository(`${server.url}/.git`, p, { fetch: { depth: 1 } });
      expect(repo.isShallow()).toBe(true);
      const revwalk = repo.revwalk();
      revwalk.pushHead();
      expect(revwalk.next()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
      expect(revwalk.next()).toBeNull();
    } finally {
      await server.close();
    }
  });

  it('shallow clone is not supported from local path', async () => {
    const localPath = await useFixture('commits');
    const p = await makeTmpDir('clone');
    await expect(cloneRepository(localPath, p, { fetch: { depth: 1 } })).rejects.toThrowError(
      /shallow fetch is not supported by the local transport/
    );
  });

  it('clone from remote', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    expect(repo.state()).toBe('Clean');
  });

  it('clone from remote with callbacks', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const transferProgress = vi.fn().mockImplementation(() => true);
    await cloneRepository('https://github.com/seokju-na/dummy-repo', p, {
      fetch: {
        callbacks: {
          transferProgress,
        },
      },
    });
    await vi.waitFor(() => {
      expect(transferProgress).toHaveBeenCalled();
    });
  });

  it('get head', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const ref = repo.head();
    expect(ref.name()).toEqual('refs/heads/main');
    expect(ref.shorthand()).toEqual('main');
    expect(ref.type()).toEqual('Direct');
    expect(ref.target()).toEqual('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(ref.isBranch()).toBe(true);
    expect(ref.symbolicTarget()).toBeNull();
  });

  it('set head', async () => {
    const p = await useFixture('revparse');
    const repo = await openRepository(p);
    repo.setHead('refs/heads/other');
    const ref = repo.head();
    expect(ref.name()).toEqual('refs/heads/other');
    expect(ref.shorthand()).toEqual('other');
    expect(ref.type()).toEqual('Direct');
    expect(ref.target()).toEqual('b580e5f5030f508a3658a4155f44cdd9754950c5');
    expect(ref.isBranch()).toBe(true);
    expect(ref.symbolicTarget()).toBeNull();
  });
});
