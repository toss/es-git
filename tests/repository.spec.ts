import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it, vi } from 'vitest';
import { Repository } from '../index';
import { LINUX } from './env';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

const CI = !!process.env.CI;

describe('Repository', () => {
  it('init git repository', async () => {
    const p = await useFixture('notgit');
    Repository.init(p);
  });

  it('init git repository with options', async () => {
    const p = await useFixture('notgit');
    const repo = Repository.init(p, { bare: true, initialHead: 'my-branch' });
    expect(repo.isBare()).toBe(true);
  });

  it('open git repository', async () => {
    const p = await useFixture('empty');
    Repository.open(p);
  });

  it('error if given path is not git repository', async () => {
    const p = await useFixture('notgit');
    expect(() => Repository.open(p)).toThrowError(/libgit2 error: could not find repository/);
  });

  it('clone from local', async () => {
    const localPath = await useFixture('commits');
    const p = await makeTmpDir('clone');
    Repository.clone(localPath, p);
    await expect(fs.readFile(path.join(p, 'first'), 'utf8')).resolves.toEqual(expect.stringContaining('first'));
  });

  it('clone from remote', { skip: LINUX }, async () => {
    const p = await makeTmpDir('clone');
    const onTransferProgress = vi.fn();
    Repository.clone('https://github.com/toss/es-toolkit', p, {
      fetch: {
        onTransferProgress,
      },
    });
    expect(onTransferProgress).toHaveBeenCalled();
  });

  it('clone from remote with credential', { skip: CI || LINUX }, async () => {
    const p = await makeTmpDir('clone');
    Repository.clone('git@github.com:toss/es-toolkit', p, {
      fetch: {
        followRedirects: 'All',
        credential: {
          type: 'SSHKeyFromAgent',
        },
      },
    });
  });
});
