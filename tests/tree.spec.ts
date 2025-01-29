import { describe, expect, it, vi } from 'vitest';
import { ObjectType, openRepository } from '../index';
import { useFixture } from './fixtures';

describe('tree', () => {
  it('walk tree', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    const tree = commit.tree();
    const walk = vi.fn();
    tree.walk('PreOrder', entry => {
      walk(entry.name(), entry.id(), entry.type());
      return 0;
    });
    expect(walk.mock.calls).toEqual([
      ['first', '9c59e24b8393179a5d712de4f990178df5734d99', ObjectType.Blob],
      ['second', 'e019be006cf33489e2d0177a3837a2384eddebc5', ObjectType.Blob],
    ]);
  });
});
