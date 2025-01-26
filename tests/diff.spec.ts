import { describe, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('diff', () => {
  it('1', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const index1 = repo.index();
    const index2 = repo.index();
    const diff = repo.diffIndexToIndex(index1, index2);
    console.log([...diff.deltas()]);
    const stats = diff.stats();
    console.log(stats.deletions);
    console.log(stats.insertions);
    console.log(stats.filesChanged);
    console.log(stats.print());
  });
});
