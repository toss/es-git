import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('rebase', () => {
  it('rebase (no-conflict): feature onto main', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Test User', email: 'test@example.com' };

    // initial commit (o0)
    let index = repo.index();
    const initTreeId = index.writeTree();
    const initTree = repo.getTree(initTreeId);
    const o0 = repo.commit(initTree, 'initial', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const initCommit = repo.getCommit(o0);

    // main: add main.txt (o1)
    repo.setHead('refs/heads/main');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'main.txt'), 'main');
    index = repo.index();
    index.addPath('main.txt');
    const mainTreeId = index.writeTree();
    const mainTree = repo.getTree(mainTreeId);
    const o1 = repo.commit(mainTree, 'main-1', {
      updateRef: 'refs/heads/main',
      author: sig,
      committer: sig,
      parents: [o0],
    });

    // feature: add feat.txt (o2)
    repo.createBranch('feature', initCommit);
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'feat.txt'), 'feat');
    index = repo.index();
    index.addPath('feat.txt');
    const featTreeId = index.writeTree();
    const featTree = repo.getTree(featTreeId);
    const o2 = repo.commit(featTree, 'feat-1', {
      updateRef: 'refs/heads/feature',
      author: sig,
      committer: sig,
      parents: [o0],
    });

    // rebase feature onto main
    const featureRef = repo.getReference('refs/heads/feature');
    const mainRef = repo.getReference('refs/heads/main');
    const featureHead = repo.getAnnotatedCommitFromReference(featureRef);
    const mainHead = repo.getAnnotatedCommitFromReference(mainRef);

    const rebase = repo.rebase(featureHead, mainHead);
    expect(rebase.len()).toEqual(1n);
    const ids = [o2];
    const types = ['Pick'];
    let i = 0;
    for (const op of rebase) {
      expect(op.id).toEqual(ids[i]);
      expect(op.type).toEqual(types[i]);
      rebase.commit({ committer: sig });
      i += 1;
    }
    rebase.finish(sig);

    const featureAfter = repo.getReference('refs/heads/feature').target()!;
    const featureCommit = repo.getCommit(featureAfter);
    expect(featureCommit.message()).toBe('feat-1');
    expect(repo.getMergeBase(featureAfter, o1)).toEqual(o1);

    const mainStatus = repo.getStatusFile('main.txt');
    const featStatus = repo.getStatusFile('feat.txt');
    expect(mainStatus).toBeTruthy();
    expect(featStatus).toBeTruthy();
  });

  it('rebase (conflict): resolve, commit and finish', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Test User', email: 'test@example.com' };

    // initial with conflict.txt
    await fs.writeFile(path.join(p, 'conflict.txt'), 'base\n');
    let index = repo.index();
    index.addPath('conflict.txt');
    let treeId = index.writeTree();
    let tree = repo.getTree(treeId);
    const baseOid = repo.commit(tree, 'base', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const baseCommit = repo.getCommit(baseOid);

    // main changes conflict.txt
    repo.setHead('refs/heads/main');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'conflict.txt'), 'main-change\n');
    index = repo.index();
    index.addPath('conflict.txt');
    treeId = index.writeTree();
    tree = repo.getTree(treeId);
    const mainOid = repo.commit(tree, 'main-change', {
      updateRef: 'refs/heads/main',
      author: sig,
      committer: sig,
      parents: [baseOid],
    });

    // feature changes conflict.txt differently
    repo.createBranch('feature', baseCommit);
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'conflict.txt'), 'feature-change\n');
    index = repo.index();
    index.addPath('conflict.txt');
    treeId = index.writeTree();
    tree = repo.getTree(treeId);
    const conflictOid = repo.commit(tree, 'feature-change', {
      updateRef: 'refs/heads/feature',
      author: sig,
      committer: sig,
      parents: [baseOid],
    });

    // start rebase: feature onto main
    const featureHead = repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/feature'));
    const mainHead = repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/main'));
    const rebase = repo.rebase(featureHead, mainHead);

    // next operation should apply and produce conflict
    const iter = rebase[Symbol.iterator]();
    const first = iter.next();
    expect(first.value!.type).toEqual('Pick');
    expect(first.value!.id).toEqual(conflictOid);

    // conflict expected
    index = repo.index();
    expect(index.hasConflicts()).toBe(true);

    // resolve conflict by writing resolved content and adding to index
    await fs.writeFile(path.join(p, 'conflict.txt'), 'resolved\n');
    index.addPath('conflict.txt');
    index.write();

    // commit current patch after resolving
    const newOid = rebase.commit({ committer: sig, message: 'resolved' });
    expect(newOid).toMatch(/^[0-9a-f]{40}$/);

    // no more operations
    const second = iter.next();
    expect(second.done).toBeTruthy();

    // finish rebase
    rebase.finish(sig);
    expect(repo.state()).toBe('Clean');

    // feature tip should be rebased atop main
    const featureTip = repo.getReference('refs/heads/feature').target()!;
    expect(repo.getMergeBase(featureTip, mainOid)).toEqual(mainOid);

    // content should be resolved
    const content = await fs.readFile(path.join(p, 'conflict.txt'), 'utf8');
    expect(content).toBe('resolved\n');
  });

  it('rebase abort restores original state', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Test User', email: 'test@example.com' };

    // setup base/main/feature like before, but abort after conflict
    await fs.writeFile(path.join(p, 'x.txt'), 'base\n');
    let index = repo.index();
    index.addPath('x.txt');
    let treeId = index.writeTree();
    let tree = repo.getTree(treeId);
    const baseOid = repo.commit(tree, 'base', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const baseCommit = repo.getCommit(baseOid);

    repo.setHead('refs/heads/main');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'x.txt'), 'main\n');
    index = repo.index();
    index.addPath('x.txt');
    treeId = index.writeTree();
    tree = repo.getTree(treeId);
    const mainOid = repo.commit(tree, 'm', {
      updateRef: 'refs/heads/main',
      author: sig,
      committer: sig,
      parents: [baseOid],
    });

    repo.createBranch('feature', baseCommit);
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'x.txt'), 'feature\n');
    index = repo.index();
    index.addPath('x.txt');
    treeId = index.writeTree();
    tree = repo.getTree(treeId);
    const featureOidBefore = repo.commit(tree, 'f', {
      updateRef: 'refs/heads/feature',
      author: sig,
      committer: sig,
      parents: [baseOid],
    });

    const featureHead = repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/feature'));
    const mainHead = repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/main'));
    const rebase = repo.rebase(featureHead, mainHead, undefined, { quiet: true });

    // advance once to create conflict and then abort
    const it = rebase[Symbol.iterator]();
    const step = it.next();
    expect(step.done).toBeFalsy();

    rebase.abort(); // abort rebase
    expect(repo.state()).toBe('Clean');

    // feature branch should still point to original (un-rebased) tip
    const featureTipAfterAbort = repo.getReference('refs/heads/feature').target()!;
    expect(featureTipAfterAbort).toEqual(featureOidBefore);

    // main tip unchanged
    const mainTipAfterAbort = repo.getReference('refs/heads/main').target()!;
    expect(mainTipAfterAbort).toEqual(mainOid);
  });

  it('openRebase resumes ongoing rebase', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const sig = { name: 'Test User', email: 'test@example.com' };

    // simple two-commit rebase chain to observe resume behavior
    let index = repo.index();
    const initTreeId = index.writeTree();
    const initTree = repo.getTree(initTreeId);
    const baseOid = repo.commit(initTree, 'init', {
      updateRef: 'HEAD',
      author: sig,
      committer: sig,
      parents: [repo.head().target()!],
    });
    const baseCommit = repo.getCommit(baseOid);

    // main adds A
    repo.setHead('refs/heads/main');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'A'), 'A');
    index = repo.index();
    index.addPath('A');
    let t = repo.getTree(index.writeTree());
    repo.commit(t, 'A', { updateRef: 'refs/heads/main', author: sig, committer: sig, parents: [baseOid] });

    // feature adds B then C (two ops to apply)
    repo.createBranch('feature', baseCommit);
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();
    await fs.writeFile(path.join(p, 'B'), 'B');
    index = repo.index();
    index.addPath('B');
    t = repo.getTree(index.writeTree());
    const bOid = repo.commit(t, 'B', {
      updateRef: 'refs/heads/feature',
      author: sig,
      committer: sig,
      parents: [baseOid],
    });

    await fs.writeFile(path.join(p, 'C'), 'C');
    index = repo.index();
    index.addPath('C');
    t = repo.getTree(index.writeTree());
    repo.commit(t, 'C', { updateRef: 'refs/heads/feature', author: sig, committer: sig, parents: [bOid] });

    const rb = repo.rebase(
      repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/feature')),
      repo.getAnnotatedCommitFromReference(repo.getReference('refs/heads/main'))
    );
    const i = rb[Symbol.iterator]();
    const first = i.next();
    expect(first.done).toBeFalsy();
    rb.commit({ committer: sig });

    const resumed = repo.openRebase();
    expect(resumed.len()).toEqual(2n);
    const remainingFirst = resumed[Symbol.iterator]().next();
    expect(remainingFirst.done).toBeFalsy();

    for (const _op of resumed) {
      resumed.commit({ committer: sig });
    }
    resumed.finish(sig);

    expect(repo.state()).toBe('Clean');
  });
});
