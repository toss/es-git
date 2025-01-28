import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { isValidOid, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('commit', () => {
  it('get commit', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(commit.author().name).toEqual('Seokju Na');
    expect(commit.author().email).toEqual('seokju.me@gmail.com');
    expect(commit.author().timestamp).toEqual(1732957216);
    expect(commit.message()).toEqual('second\n');
    expect(commit.summary()).toEqual('second');
    expect(commit.body()).toEqual(null);
    expect(commit.time().toISOString()).toEqual('2024-11-30T09:00:16.000Z');
  });

  it('returns null if oid of commit does not exists', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const commit = repo.findCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(commit).toBeNull();
  });

  it('commit on head', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'third'), 'third');
    const index = repo.index();
    index.addPath('third');
    index.write();
    const tree = repo.head().peelToTree();
    const oid = repo.commit(tree, 'test commit', {
      author: {
        name: 'Seokju Na',
        email: 'seokju.me@toss.im',
      },
      committer: {
        name: 'Seokju Na',
        email: 'seokju.me@toss.im',
      },
    });
    expect(isValidOid(oid)).toBe(true);
  });
});
