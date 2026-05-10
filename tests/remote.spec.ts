import { describe, expect, it, vi } from 'vitest';
import { cloneRepository, initRepository, openRepository } from '../index';
import { isTarget } from './env';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

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
    const updateTips = vi.fn().mockImplementation(() => true);
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
