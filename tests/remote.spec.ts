import { describe, expect, it } from 'vitest';
import { LINUX } from './env';
import { makeTmpDir } from './tmp';
import { cloneRepository, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('remote', () => {
  it('get remote names', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/toss/es-toolkit', p);
    expect(repo.remoteNames()).toContain('origin');
  });

  it('get remote object', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/toss/es-toolkit', p);
    const remote = repo.getRemote('origin');
    expect(remote.name).toEqual('origin');
    expect(remote.url).toEqual('https://github.com/toss/es-toolkit');
    expect(() => repo.getRemote('not_exists')).toThrowError(/libgit2 error: remote 'not_exists' does not exist/);
  });

  it('create remote', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const remote = repo.createRemote('origin', 'git@github.com:toss/empty.git');
    expect(remote.name).toEqual('origin');
  });

  it('create remote with fetch refspec', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const remote = repo.createRemote('origin', 'git@github.com:toss/empty.git', {
      fetchRefspec: '+refs/*:refs/*'
    });
    expect(remote.name).toEqual('origin');
  });

  it('fetch remote', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/toss/es-toolkit', p);
    await repo.fetchRemote('origin', ['main']);
  });

  it('get remote default branch', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/toss/es-toolkit', p);
    const branch = await repo.getRemoteDefaultBranch('origin');
    expect(branch).toEqual('refs/heads/main');
  });
});
