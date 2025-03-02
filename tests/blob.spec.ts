import { describe, expect, it } from 'vitest';
import { useFixture } from './fixtures';
import { openRepository } from '../index';

describe('blob', () => {
  it('get blob from git object', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const blob = commit.tree().getName('second')?.toObject(repo)?.peelToBlob();
    expect(blob?.content()).toEqual(new TextEncoder().encode('second\n'));
  });
});
