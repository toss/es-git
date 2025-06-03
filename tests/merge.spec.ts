import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('merge', () => {
  it('get merge base', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };

    /**
     * Create the following:
     *    /---o4
     *   /---o3
     * o1---o2
     */
    let index = repo.index();
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, 'first', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const firstCommit = repo.getCommit(firstOid);

    repo.createBranch('branch_a', firstCommit);
    repo.createBranch('branch_b', firstCommit);
    repo.createBranch('branch_c', firstCommit);

    index = repo.index();
    await fs.writeFile(path.join(p, 'file_a'), 'A');
    index.addPath('file_a');
    const aTreeId = index.writeTree();
    const aTree = repo.getTree(aTreeId);
    const aOid = repo.commit(aTree, 'a', {
      updateRef: 'refs/heads/branch_a',
      author: sig,
      committer: sig,
      parents: [firstOid],
    });

    index = repo.index();
    await fs.writeFile(path.join(p, 'file_b'), 'B');
    index.addPath('file_b');
    const bTreeId = index.writeTree();
    const bTree = repo.getTree(bTreeId);
    const bOid = repo.commit(bTree, 'b', {
      updateRef: 'refs/heads/branch_b',
      author: sig,
      committer: sig,
      parents: [firstOid],
    });

    index = repo.index();
    await fs.writeFile(path.join(p, 'file_c'), 'C');
    index.addPath('file_c');
    const cTreeId = index.writeTree();
    const cTree = repo.getTree(cTreeId);
    const cOid = repo.commit(cTree, 'b', {
      updateRef: 'refs/heads/branch_c',
      author: sig,
      committer: sig,
      parents: [firstOid],
    });

    let merged = repo.getMergeBase(aOid, bOid);
    expect(merged).toEqual(firstOid);

    merged = repo.getMergeBaseMany([aOid, bOid, cOid]);
    expect(merged).toEqual(firstOid);

    merged = repo.getMergeBaseOctopus([aOid, bOid, cOid]);
    expect(merged).toEqual(firstOid);
  });

  it('analyze merge fast-forward', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };

    let index = repo.index();
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, '1', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const firstCommit = repo.getCommit(firstOid);

    index = repo.index();
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, '2', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const secondCommit = repo.getCommit(secondOid);

    repo.setHeadDetached(secondCommit);
    repo.createBranch('other', secondCommit);
    repo.createBranch('main', firstCommit, { force: true });
    repo.setHead('refs/heads/main');

    const targetCommit = repo.getAnnotatedCommit(secondCommit);
    const { analysis } = repo.analyzeMergeForRef(repo.getReference('refs/heads/main'), [targetCommit]);
    expect(analysis.fastForward).toBe(true);
  });
});
