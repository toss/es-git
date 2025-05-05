import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('checkout', () => {
  it('smoke checkout', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    repo.checkoutHead();
  });

  it('checkout branches', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    let index = repo.index();
    const treeId = index.writeTree();
    const tree = repo.getTree(treeId);
    const firstOid = repo.commit(tree, '1', {
      updateRef: 'HEAD',
      parents: [repo.head().target()!],
    });
    const firstCommit = repo.getCommit(firstOid);

    repo.createBranch('branch-a', firstCommit);
    repo.createBranch('branch-b', firstCommit);

    await fs.writeFile(path.join(p, 'file1'), 'A');
    index = repo.index();
    index.addPath('file1');
    const treeAId = index.writeTree();
    const treeA = repo.getTree(treeAId);
    const aOid = repo.commit(treeA, 'A', {
      updateRef: 'refs/heads/branch-a',
      parents: [firstOid],
    });
    repo.setHead('refs/heads/branch-a');
    repo.checkoutHead();
    expect(repo.head().target()).toEqual(aOid);

    await fs.writeFile(path.join(p, 'file2'), 'B');
    index = repo.index();
    index.addPath('file2');
    const treeBId = index.writeTree();
    const treeB = repo.getTree(treeBId);
    const bOid = repo.commit(treeB, 'B', {
      updateRef: 'refs/heads/branch-b',
      parents: [firstOid],
    });
    repo.setHead('refs/heads/branch-b');
    repo.checkoutHead();
    expect(repo.head().target()).toEqual(bOid);
  });

  it('conflict error when index is changed', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'added.txt'), 'added');
    const index = repo.index();
    index.addPath('added.txt');
    index.write();

    expect(() => repo.checkoutIndex()).toThrowError();
    expect(() => repo.checkoutIndex(index)).toThrowError();
  });
});
