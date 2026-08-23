import { describe, expect, it, vi } from 'vitest';
import { cloneRepository, initRepository, openRepository } from '../index';
import { isTarget } from './env';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

async function createLocalRemote() {
  const localPath = await makeTmpDir('credential-validation-local');
  const remotePath = await makeTmpDir('credential-validation-remote');
  const repo = await initRepository(localPath);
  await initRepository(remotePath, { bare: true });
  return repo.createRemote('origin', remotePath);
}

describe('remote', () => {
  it('get remote names', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    expect(repo.remoteNames()).toContain('origin');
  });

  it('get remote', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    const remote = repo.getRemote('origin');
    expect(remote.name()).toEqual('origin');
    expect(remote.url()).toEqual('https://github.com/seokju-na/dummy-repo');
    expect(() => repo.getRemote('not_exists')).toThrowError(/libgit2 error: remote 'not_exists' does not exist/);
  });

  it('create remote', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const remote = repo.createRemote('origin', 'git@github.com:toss/empty.git');
    expect(remote.name()).toEqual('origin');
  });

  it('create remote with fetch refspec', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const remote = repo.createRemote('origin', 'git@github.com:toss/empty.git', {
      fetchRefspec: '+refs/*:refs/*',
    });
    expect(remote.name()).toEqual('origin');
  });

  it('fetch remote', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    const remote = repo.getRemote('origin');
    await remote.fetch(['main']);
  });

  it('get remote default branch', { skip: isTarget('linux', undefined, 'gnu') }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    const remote = repo.getRemote('origin');
    const branch = await remote.defaultBranch();
    expect(branch).toEqual('refs/heads/main');
  });

  it('push remote to local bare repository', async () => {
    const localPath = await useFixture('commits');
    const remotePath = await makeTmpDir('remote-bare');
    const localRepo = await openRepository(localPath);
    const remoteRepo = await initRepository(remotePath, { bare: true });

    const remote = localRepo.createRemote('origin', remotePath);
    const localHead = localRepo.head().target()!;

    await remote.push(['refs/heads/main:refs/heads/main']);

    expect(remoteRepo.getReference('refs/heads/main').target()).toEqual(localHead);
  });

  it('push remote with explicit refspec', async () => {
    const localPath = await useFixture('commits');
    const remotePath = await makeTmpDir('remote-bare');
    const localRepo = await openRepository(localPath);
    const remoteRepo = await initRepository(remotePath, { bare: true });

    const remote = localRepo.createRemote('origin', remotePath);
    const localHead = localRepo.head().target()!;

    await remote.push(['refs/heads/main:refs/heads/pushed-main']);

    expect(remoteRepo.getReference('refs/heads/pushed-main').target()).toEqual(localHead);
  });

  it('push remote with callbacks', async () => {
    const localPath = await useFixture('commits');
    const remotePath = await makeTmpDir('remote-bare');
    const localRepo = await openRepository(localPath);
    const remoteRepo = await initRepository(remotePath, { bare: true });

    const remote = localRepo.createRemote('origin', remotePath);
    const localHead = localRepo.head().target()!;
    const updateTips = vi.fn();
    const pushUpdateReference = vi.fn();
    const pushTransferProgress = vi.fn();
    const pushNegotiation = vi.fn();
    const packProgress = vi.fn();

    await remote.push(['refs/heads/main:refs/heads/main'], {
      callbacks: {
        updateTips,
        pushUpdateReference,
        pushTransferProgress,
        pushNegotiation,
        packProgress,
      },
    });

    expect(remoteRepo.getReference('refs/heads/main').target()).toEqual(localHead);
    await vi.waitFor(() => {
      expect(updateTips).toHaveBeenCalledWith(
        'refs/remotes/origin/main',
        '0000000000000000000000000000000000000000',
        localHead
      );
      expect(pushUpdateReference).toHaveBeenCalledWith('refs/heads/main', null);
      expect(pushNegotiation).toHaveBeenCalledWith([
        {
          src: '0000000000000000000000000000000000000000',
          dst: localHead,
          srcRefname: 'refs/heads/main',
          dstRefname: 'refs/heads/main',
        },
      ]);
      expect(packProgress).toHaveBeenCalled();
    });
  });

  it.each([
    {
      credential: { type: 'SSHKeyFromPath' },
      message: 'credential.privateKeyPath is required for SSHKeyFromPath credentials',
    },
    {
      credential: { type: 'SSHKey' },
      message: 'credential.privateKey is required for SSHKey credentials',
    },
    {
      credential: { type: 'Plain' },
      message: 'credential.password is required for Plain credentials',
    },
  ])('rejects missing required fields for $credential.type credentials', async ({ credential, message }) => {
    const remote = await createLocalRemote();
    await expect(remote.fetch([], { fetch: { credential: credential as any } })).rejects.toMatchObject({
      code: 'InvalidArg',
      message,
    });
  });

  it.each([
    { type: 'SSHKeyFromPath', privateKeyPath: 'synthetic-private-key-path' },
    { type: 'SSHKey', privateKey: 'synthetic-private-key' },
    { type: 'Plain', password: 'synthetic-password' },
  ])('accepts valid $type credentials', async credential => {
    const remote = await createLocalRemote();
    await expect(remote.fetch([], { fetch: { credential: credential as any } })).resolves.toBeUndefined();
  });
});
