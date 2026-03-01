import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('revert', () => {
  const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };

  it('revert simple commit', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'file.txt'), 'initial content');
    let index = repo.index();
    index.addPath('file.txt');
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, 'first commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'file.txt'), 'modified content');
    index = repo.index();
    index.addPath('file.txt');
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, 'second commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [firstOid],
    });

    const commitToRevert = repo.getCommit(secondOid);
    repo.revert(commitToRevert);

    const content = await fs.readFile(path.join(p, 'file.txt'), 'utf-8');
    expect(content).toBe('initial content');

    repo.cleanupState();
    expect(repo.state()).toBe('Clean');
  });

  it('revert with options', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'test.txt'), 'version 1');
    let index = repo.index();
    index.addPath('test.txt');
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, 'initial', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'test.txt'), 'version 2');
    index = repo.index();
    index.addPath('test.txt');
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, 'update', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [firstOid],
    });

    const commitToRevert = repo.getCommit(secondOid);
    repo.revert(commitToRevert, {
      mergeOptions: {
        findRenames: true,
        failOnConflict: false,
      },
    });

    const content = await fs.readFile(path.join(p, 'test.txt'), 'utf-8');
    expect(content).toBe('version 1');

    repo.cleanupState();
  });

  it('revert file addition', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'existing.txt'), 'existing file');
    let index = repo.index();
    index.addPath('existing.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'added.txt'), 'new file content');
    index = repo.index();
    index.addPath('added.txt');
    const addTreeId = index.writeTree();
    const addTree = repo.getTree(addTreeId);
    const addOid = repo.commit(addTree, 'add new file', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    const commitToRevert = repo.getCommit(addOid);
    repo.revert(commitToRevert);

    repo.cleanupState();

    const fileExists = await fs
      .access(path.join(p, 'added.txt'))
      .then(() => true)
      .catch(() => false);
    expect(fileExists).toBe(false);

    const existingFileContent = await fs.readFile(path.join(p, 'existing.txt'), 'utf-8');
    expect(existingFileContent).toBe('existing file');
  });

  it('revert file deletion', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'to-delete.txt'), 'file to be deleted');
    let index = repo.index();
    index.addPath('to-delete.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base with file', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.unlink(path.join(p, 'to-delete.txt'));
    index = repo.index();
    index.removePath('to-delete.txt');
    const deleteTreeId = index.writeTree();
    const deleteTree = repo.getTree(deleteTreeId);
    const deleteOid = repo.commit(deleteTree, 'delete file', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    const commitToRevert = repo.getCommit(deleteOid);
    repo.revert(commitToRevert);

    repo.cleanupState();

    const fileContent = await fs.readFile(path.join(p, 'to-delete.txt'), 'utf-8');
    expect(fileContent).toBe('file to be deleted');
  });

  it('revert state is clean after cleanup', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'state-test.txt'), 'initial');
    let index = repo.index();
    index.addPath('state-test.txt');
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, 'first', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'state-test.txt'), 'modified');
    index = repo.index();
    index.addPath('state-test.txt');
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, 'second', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [firstOid],
    });

    expect(repo.state()).toBe('Clean');

    const commitToRevert = repo.getCommit(secondOid);
    repo.revert(commitToRevert);

    repo.cleanupState();
    expect(repo.state()).toBe('Clean');

    repo.cleanupState();
    expect(repo.state()).toBe('Clean');
  });

  it('revert keeps HEAD unchanged', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'head.txt'), 'v1');
    let index = repo.index();
    index.addPath('head.txt');
    const t1 = repo.getTree(index.writeTree());
    const c1 = repo.commit(t1, 'v1', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    await fs.writeFile(path.join(p, 'head.txt'), 'v2');
    index = repo.index();
    index.addPath('head.txt');
    const t2 = repo.getTree(index.writeTree());
    const c2 = repo.commit(t2, 'v2', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [c1],
    });

    const before = repo.head().target();
    repo.revert(repo.getCommit(c2));
    const after = repo.head().target();
    expect(after).toBe(before);
  });

  it('revert merge commit with mainline 1', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'base.txt'), 'base content');
    let index = repo.index();
    index.addPath('base.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    const baseCommit = repo.getCommit(baseOid);

    repo.createBranch('branch-a', baseCommit);
    repo.setHead('refs/heads/branch-a');
    await fs.writeFile(path.join(p, 'file-a.txt'), 'content A');
    index = repo.index();
    index.addPath('file-a.txt');
    const aTreeId = index.writeTree();
    const aTree = repo.getTree(aTreeId);
    const aOid = repo.commit(aTree, 'commit A', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });
    const aCommit = repo.getCommit(aOid);

    repo.createBranch('branch-b', baseCommit);
    repo.setHead('refs/heads/branch-b');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'file-b.txt'), 'content B');
    index = repo.index();
    index.addPath('file-b.txt');
    const bTreeId = index.writeTree();
    const bTree = repo.getTree(bTreeId);
    const bOid = repo.commit(bTree, 'commit B', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });
    const bCommit = repo.getCommit(bOid);

    repo.setHead('refs/heads/branch-a');
    repo.checkoutHead({ force: true });

    await fs.writeFile(path.join(p, 'file-b.txt'), 'content B');
    index = repo.index();
    index.addPath('file-a.txt');
    index.addPath('file-b.txt');
    const mergeTreeId = index.writeTree();
    const mergeTree = repo.getTree(mergeTreeId);
    const mergeOid = repo.commit(mergeTree, 'merge A and B', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [aOid, bOid],
    });
    const mergeCommit = repo.getCommit(mergeOid);

    repo.revert(mergeCommit, {
      mainline: 1,
    });

    repo.cleanupState();

    const fileBExists = await fs
      .access(path.join(p, 'file-b.txt'))
      .then(() => true)
      .catch(() => false);
    expect(fileBExists).toBe(false);

    const fileAExists = await fs
      .access(path.join(p, 'file-a.txt'))
      .then(() => true)
      .catch(() => false);
    expect(fileAExists).toBe(true);
  });

  it('error on merge revert without mainline', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'base.txt'), 'base');
    let index = repo.index();
    index.addPath('base.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    const baseCommit = repo.getCommit(baseOid);
    repo.createBranch('feature', baseCommit);

    await fs.writeFile(path.join(p, 'main.txt'), 'main');
    index = repo.index();
    index.addPath('main.txt');
    const mainTreeId = index.writeTree();
    const mainTree = repo.getTree(mainTreeId);
    const mainOid = repo.commit(mainTree, 'main commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    await fs.writeFile(path.join(p, 'feature.txt'), 'feature');
    index = repo.index();
    index.addPath('feature.txt');
    const featureTreeId = index.writeTree();
    const featureTree = repo.getTree(featureTreeId);
    const featureOid = repo.commit(featureTree, 'feature commit', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    index = repo.index();
    await fs.writeFile(path.join(p, 'main.txt'), 'main');
    await fs.writeFile(path.join(p, 'feature.txt'), 'feature');
    index.addPath('main.txt');
    index.addPath('feature.txt');
    const mergeTreeId = index.writeTree();
    const mergeTree = repo.getTree(mergeTreeId);
    const mergeOid = repo.commit(mergeTree, 'merge', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [mainOid, featureOid],
    });

    const mergeCommit = repo.getCommit(mergeOid);

    expect(() => {
      repo.revert(mergeCommit);
    }).toThrow();
  });

  it('revert merge commit with mainline 2', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'base.txt'), 'base');
    let index = repo.index();
    index.addPath('base.txt');
    const baseTree = repo.getTree(index.writeTree());
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    const baseCommit = repo.getCommit(baseOid);

    repo.createBranch('A', baseCommit);
    repo.setHead('refs/heads/A');
    await fs.writeFile(path.join(p, 'a.txt'), 'A');
    index = repo.index();
    index.addPath('a.txt');
    const aTree = repo.getTree(index.writeTree());
    const aOid = repo.commit(aTree, 'A', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    repo.createBranch('B', baseCommit);
    repo.setHead('refs/heads/B');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'b.txt'), 'B');
    index = repo.index();
    index.addPath('b.txt');
    const bTree = repo.getTree(index.writeTree());
    const bOid = repo.commit(bTree, 'B', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    repo.setHead('refs/heads/A');
    repo.checkoutHead({ force: true });

    await fs.writeFile(path.join(p, 'b.txt'), 'B');
    index = repo.index();
    index.addPath('a.txt');
    index.addPath('b.txt');
    const mergeTree = repo.getTree(index.writeTree());
    const mergeOid = repo.commit(mergeTree, 'merge', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [aOid, bOid],
    });
    const mergeCommit = repo.getCommit(mergeOid);

    repo.revert(mergeCommit, { mainline: 2 });
    repo.cleanupState();

    const aExists = await fs
      .access(path.join(p, 'a.txt'))
      .then(() => true)
      .catch(() => false);
    const bExists = await fs
      .access(path.join(p, 'b.txt'))
      .then(() => true)
      .catch(() => false);
    expect(aExists).toBe(false);
    expect(bExists).toBe(true);
  });

  it('handle conflicts during revert', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nline2\nline3');
    let index = repo.index();
    index.addPath('conflict.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nmodified2\nline3');
    index = repo.index();
    index.addPath('conflict.txt');
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, 'modify middle', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), 'changed1\nchanged2\nchanged3');
    index = repo.index();
    index.addPath('conflict.txt');
    const thirdTreeId = index.writeTree();
    const thirdTree = repo.getTree(thirdTreeId);
    const thirdOid = repo.commit(thirdTree, 'change all', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [secondOid],
    });

    const secondCommit = repo.getCommit(secondOid);
    const thirdCommit = repo.getCommit(thirdOid);

    const revertIndex = repo.revertCommit(secondCommit, thirdCommit, 0, { failOnConflict: false });

    expect(revertIndex.hasConflicts()).toBe(true);
  });

  it('revert conflict throws with failOnConflict', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\n2\n3');
    let index = repo.index();
    index.addPath('conflict.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\nA\n3');
    index = repo.index();
    index.addPath('conflict.txt');
    const aTreeId = index.writeTree();
    const aTree = repo.getTree(aTreeId);
    const aOid = repo.commit(aTree, 'A changes', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\nB\n3');
    index = repo.index();
    index.addPath('conflict.txt');
    const bTreeId = index.writeTree();
    const bTree = repo.getTree(bTreeId);
    repo.commit(bTree, 'B changes', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [aOid],
    });

    const commitA = repo.getCommit(aOid);
    expect(() =>
      repo.revert(commitA, {
        mergeOptions: { failOnConflict: true },
      })
    ).toThrow();
  });

  it('revert conflict leaves conflicts without failOnConflict', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\n2\n3');
    let index = repo.index();
    index.addPath('conflict.txt');
    const baseTreeId = index.writeTree();
    const baseTree = repo.getTree(baseTreeId);
    const baseOid = repo.commit(baseTree, 'base', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\nA\n3');
    index = repo.index();
    index.addPath('conflict.txt');
    const aTreeId = index.writeTree();
    const aTree = repo.getTree(aTreeId);
    const aOid = repo.commit(aTree, 'A changes', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [baseOid],
    });

    await fs.writeFile(path.join(p, 'conflict.txt'), '1\nB\n3');
    index = repo.index();
    index.addPath('conflict.txt');
    const bTreeId = index.writeTree();
    const bTree = repo.getTree(bTreeId);
    repo.commit(bTree, 'B changes', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [aOid],
    });

    const commitA = repo.getCommit(aOid);
    repo.revert(commitA, { mergeOptions: { failOnConflict: false } });
    expect(repo.index().hasConflicts()).toBe(true);
  });

  it('revert with checkoutOptions.dryRun does not touch working tree', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'dry.txt'), 'v1');
    let index = repo.index();
    index.addPath('dry.txt');
    const t1 = repo.getTree(index.writeTree());
    const c1 = repo.commit(t1, 'v1', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    await fs.writeFile(path.join(p, 'dry.txt'), 'v2');
    index = repo.index();
    index.addPath('dry.txt');
    const t2 = repo.getTree(index.writeTree());
    const c2 = repo.commit(t2, 'v2', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [c1],
    });

    const commitToRevert = repo.getCommit(c2);
    repo.revert(commitToRevert, { checkoutOptions: { dryRun: true } });

    const content = await fs.readFile(path.join(p, 'dry.txt'), 'utf-8');
    expect(content).toBe('v2');
    expect(repo.state()).toBe('Revert');
    repo.cleanupState();
    expect(repo.state()).toBe('Clean');
  });

  // revertCommit usage
  it('revertCommit returns index for manual control', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'manual.txt'), 'original');
    let index = repo.index();
    index.addPath('manual.txt');
    const firstTreeId = index.writeTree();
    const firstTree = repo.getTree(firstTreeId);
    const firstOid = repo.commit(firstTree, 'first', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    const firstCommit = repo.getCommit(firstOid);

    await fs.writeFile(path.join(p, 'manual.txt'), 'changed');
    index = repo.index();
    index.addPath('manual.txt');
    const secondTreeId = index.writeTree();
    const secondTree = repo.getTree(secondTreeId);
    const secondOid = repo.commit(secondTree, 'second', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [firstOid],
    });
    const secondCommit = repo.getCommit(secondOid);

    await fs.writeFile(path.join(p, 'manual.txt'), 'changed more');
    index = repo.index();
    index.addPath('manual.txt');
    const thirdTreeId = index.writeTree();
    const thirdTree = repo.getTree(thirdTreeId);
    const thirdOid = repo.commit(thirdTree, 'third', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [secondOid],
    });
    const thirdCommit = repo.getCommit(thirdOid);

    const revertIndex = repo.revertCommit(secondCommit, thirdCommit, 0);

    expect(revertIndex).toBeDefined();
    expect(revertIndex.constructor.name).toBe('Index');
  });

  it('revertCommit end-to-end via checkoutIndex', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'e2e.txt'), 'v1');
    let index = repo.index();
    index.addPath('e2e.txt');
    const t1 = repo.getTree(index.writeTree());
    const c1 = repo.commit(t1, 'v1', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    await fs.writeFile(path.join(p, 'e2e.txt'), 'v2');
    index = repo.index();
    index.addPath('e2e.txt');
    const t2 = repo.getTree(index.writeTree());
    const c2 = repo.commit(t2, 'v2', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [c1],
    });

    const idx = repo.revertCommit(repo.getCommit(c2), repo.getCommit(c2), 0);
    repo.checkoutIndex(idx);

    const content = await fs.readFile(path.join(p, 'e2e.txt'), 'utf-8');
    expect(content).toBe('v1');
  });
});
