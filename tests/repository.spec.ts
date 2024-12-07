import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { cloneRepository, initRepository, openRepository } from '../index';
import { CI, LINUX } from './env';
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

  it('clone from remote', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('https://github.com/toss/es-toolkit', p);
    expect(repo.state()).toBe('Clean');
  });

  it('clone from remote with credential', { skip: CI || LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const repo = await cloneRepository('git@github.com:toss/es-toolkit', p, {
      fetch: {
        followRedirects: 'All',
        credential: {
          type: 'SSHKeyFromAgent',
        },
      },
    });
    expect(repo.state()).toBe('Clean');
  });
});
