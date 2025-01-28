import { describe, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('tree', () => {
  it('', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const tree = commit.tree();
    tree.walk('PreOrder', entry => {
      console.log(entry.name(), entry.id(), entry.type());
      return 0;
    });
  });
});
