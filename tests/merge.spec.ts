import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('merge', () => {
  it('merge base', async () => {

  });

  it('merge analysis fast forward', async () => {
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
    const mainBranch = repo.createBranch('main', firstCommit, { force: true });
    repo.setHead('refs/heads/main');

    const targetCommit = repo.getAnnotatedCommit(secondCommit);
    const { analysis } = repo.mergeAnalysisForRef(repo.getReference('refs/heads/main'), [targetCommit]);
    expect(analysis.fastForward).toBe(true);
  });
});
