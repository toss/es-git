import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { cloneRepository, initRepository, openRepository } from '../index';
import { CI, TARGET } from './env';
import { useFixture } from './fixtures';
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

  it('error if given path is not git repository', async () => {
    const p = await useFixture('notgit');
    await expect(openRepository(p)).rejects.toThrowError(/libgit2 error: could not find repository/);
  });

  it('clone from local', async () => {
    const localPath = await useFixture('commits');
    const p = await makeTmpDir('clone');
    await cloneRepository(localPath, p);
    await expect(fs.readFile(path.join(p, 'first'), 'utf8')).resolves.toEqual(expect.stringContaining('first'));
  });

  it('clone from remote', { skip: TARGET[0] === 'linux' && TARGET[2] === 'gnu' }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo', p);
    expect(repo.state()).toBe('Clean');
  });

  it('clone from remote with credential', { skip: !CI }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/seokju-na/dummy-repo-private', p, {
      fetch: {
        followRedirects: 'All',
        credential: {
          type: 'Plain',
          password: process.env.TEST_GITHUB_TOKEN!,
        },
      },
    });
    expect(repo.state()).toBe('Clean');
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
