import { describe, expect, it } from 'vitest';
import { cloneRepository, openRepository } from '../index';
import { TARGET } from './env';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

describe('remote', () => {
  const isLinuxGnu = TARGET[0] === 'linux' && TARGET[2] === 'gnu';

  it('get remote names', { skip: isLinuxGnu }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    expect(repo.remoteNames()).toContain('origin');
  });

  it('get remote', { skip: isLinuxGnu }, async () => {
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

  it('fetch remote', { skip: isLinuxGnu }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    const remote = repo.getRemote('origin');
    await remote.fetch(['main']);
  });

  it('get remote default branch', { skip: isLinuxGnu }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    const remote = repo.getRemote('origin');
    const branch = await remote.defaultBranch();
    expect(branch).toEqual('refs/heads/main');
  });
});
