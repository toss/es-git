import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('cherrypick', () => {
  const signature = { name: 'racgoo', email: 'racgoo@example.com' };

  it('simple cherrypick ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    /**
     * Create the following: (c2' is the cherrypick of c2)
     *
     * Before cherrypick
     *   /---c3(feature)
     * c1---c2(main)
     *
     * After cherrypick
     *   /---c3---c2'(feature)
     * c1---c2(main)
     */

    // Commit c1 in main branch
    let index = repo.index();
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Commit c2 with file in main branch after c1
    index = repo.index();
    await fs.writeFile(path.join(p, 'c2_file'), 'c2_content');
    index.addPath('c2_file');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'c2', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Set head to detached state to clean up index
    repo.setHeadDetached(repo.getCommit(c1_oid));
    repo.checkoutHead({ force: true });

    // Commit c3 in feature branch
    index = repo.index();
    const c3_tree = repo.getTree(index.writeTree());
    const c3_oid = repo.commit(c3_tree, 'c3', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Cherrypick c2 commit to feature branch as c2'
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });

    // Cherrypick c2 commit to feature branch
    repo.cherrypick(repo.getCommit(c2_oid));

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    // Verify that the file content in the feature branch is the same as the original file content
    expect(await fs.readFile(path.join(p, 'c2_file'), 'utf-8')).toBe('c2_content');

    // Commit c2' in feature branch
    index = repo.index();
    const c2_prime_tree = repo.getTree(index.writeTree());
    repo.commit(c2_prime_tree, "c2'", {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c3_oid],
    });

    // Verify that the commit message in the feature branch is the same as the original commit message
    expect(repo.getCommit(repo.head().target()!).message()).toBe("c2'");

    // Clean up the state of the repository
    repo.cleanupState();
  });

  it('cherrypick merge commit with mainline ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    /**
     * Create the following:
     *
     * Base Commit Tree
     *   /---c2(branch-a)---\
     * c1----c3(branch-b)-----merge(branch-a)
     *  \----c4(branch-c)
     *
     *
     * Cherrypick merge commit to branch-b and branch-c
     *
     * After cherrypick
     *   /---c2(branch-a)---\
     * c1----c3(branch-b)-----merge(branch-a)---\
     *  \----c4(branch-c)-------------------c5(branch-c: cherrypick c2)-------c6(branch-c: cherrypick c3)
     */

    // Commit c1 in main branch
    let index = repo.index();
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    const c1_commit = repo.getCommit(c1_oid);

    // Create branch-a and commit c2
    repo.createBranch('branch-a', c1_commit);
    repo.setHead('refs/heads/branch-a');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'file-a.txt'), 'content A');
    index = repo.index();
    index.addPath('file-a.txt');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'commit A', {
      updateRef: 'refs/heads/branch-a',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Create branch-b and commit c3
    repo.createBranch('branch-b', c1_commit);
    repo.setHead('refs/heads/branch-b');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'file-b.txt'), 'content B');
    index = repo.index();
    index.addPath('file-b.txt');
    const c3_tree = repo.getTree(index.writeTree());
    const c3_oid = repo.commit(c3_tree, 'commit B', {
      updateRef: 'refs/heads/branch-b',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Create branch-c and commit c4
    repo.createBranch('branch-c', c1_commit);
    repo.setHead('refs/heads/branch-c');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'file-c.txt'), 'content C');
    index = repo.index();
    index.addPath('file-c.txt');
    const c4_tree = repo.getTree(index.writeTree());
    repo.commit(c4_tree, 'commit C', {
      updateRef: 'refs/heads/branch-c',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Merge branch-b into branch-a (create merge commit)
    repo.setHead('refs/heads/branch-a');
    repo.checkoutHead({ force: true });
    repo.merge([repo.getAnnotatedCommit(repo.getCommit(repo.getReference('refs/heads/branch-b').target()!))]);
    index = repo.index();
    const mergeTree = repo.getTree(index.writeTree());
    const mergeOid = repo.commit(mergeTree, 'merge commit', {
      updateRef: 'refs/heads/branch-a',
      author: signature,
      committer: signature,
      parents: [c2_oid, c3_oid],
    });
    const mergeCommit = repo.getCommit(mergeOid);

    // Verify that the state of the repository is "Merge"
    expect(repo.state()).toBe('Merge');
    repo.cleanupState();

    // Checkout branch-c
    repo.setHead('refs/heads/branch-c');
    repo.checkoutHead({ force: true });

    // Cherrypick merge commit to branch-c (mainline: 1 -> branch-b is merged into branch-c)
    repo.cherrypick(mergeCommit, {
      mainline: 1,
    });

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    // Verify file-a.txt is not in the working directory
    await expect(fs.readFile(path.join(p, 'file-a.txt'))).rejects.toThrow();
    // Verify file-b.txt is in the working directory
    await expect(fs.readFile(path.join(p, 'file-b.txt'), 'utf-8')).resolves.toBe('content B');
    // Verify file-c.txt is in the working directory
    await expect(fs.readFile(path.join(p, 'file-c.txt'), 'utf-8')).resolves.toBe('content C');

    // Commit cherrypicked c2 to branch-c as c5
    repo.commit(repo.getTree(repo.index().writeTree()), 'c5', {
      updateRef: 'refs/heads/branch-c',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Cherrypick merge commit to branch-c (mainline: 2 -> branch-a is merged into branch-c)
    repo.cherrypick(mergeCommit, {
      mainline: 2,
    });

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    //Verify file-a.txt is in the working directory
    await expect(fs.readFile(path.join(p, 'file-a.txt'), 'utf-8')).resolves.toBe('content A');

    // Commit cherrypicked c3 to branch-c as c6

    repo.commit(repo.getTree(repo.index().writeTree()), 'c6', {
      updateRef: 'refs/heads/branch-c',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Verify all-files are in the working directory
    await expect(fs.readFile(path.join(p, 'file-a.txt'), 'utf-8')).resolves.toBe('content A');
    await expect(fs.readFile(path.join(p, 'file-b.txt'), 'utf-8')).resolves.toBe('content B');
    await expect(fs.readFile(path.join(p, 'file-c.txt'), 'utf-8')).resolves.toBe('content C');

    // Clean up the state of the repository
    repo.cleanupState();
  });

  it('cherrypick with conflict ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    /**
     * Create the following:
     *
     * Base Commit Tree
     *   /---c2(feature)
     * c1---c3(main)
     *
     * c2 and c3 both modify the same file, causing conflict when cherrypicking c2 to main
     */

    //Commit c1 in main branch
    let index = repo.index();
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nline2\nline3');
    index.addPath('conflict.txt');
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Commit c2 in feature branch (modify conflict.txt)
    repo.createBranch('feature', repo.getCommit(c1_oid));
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nmodified2\nline3');
    index = repo.index();
    index.addPath('conflict.txt');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'c2', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Commit c3 in main branch (modify conflict.txt differently)
    repo.setHead('refs/heads/main');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nchanged2\nline3');
    index = repo.index();
    index.addPath('conflict.txt');
    const c3_tree = repo.getTree(index.writeTree());
    const c3_oid = repo.commit(c3_tree, 'c3', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Cherrypick c2 to main (should cause conflict)
    repo.cherrypick(repo.getCommit(c2_oid), {
      mergeOptions: {
        failOnConflict: false,
      },
    });

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    // Verify that conflicts exist in the index
    expect(repo.index().hasConflicts()).toBe(true);

    // Clean up the state of the repository
    repo.cleanupState();
  });

  it('cherrypick with options ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    /**
     * Create the following: (c2' is the cherrypick of c2)
     *
     * Before cherrypick
     *   /---c3(feature)
     * c1---c2(main)
     *
     * Test dryRun option to prevent working tree changes
     */

    // Commit c1 in main branch
    let index = repo.index();
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Commit c2 with file-a.txt in main branch after c1
    index = repo.index();
    await fs.writeFile(path.join(p, 'file-a.txt'), 'content A');
    index.addPath('file-a.txt');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'c2', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Set head to detached state to avoid merge conflicts
    repo.setHeadDetached(repo.getCommit(c1_oid));
    repo.checkoutHead({ force: true });

    // Commit c3 in feature branch
    index = repo.index();
    const c3_tree = repo.getTree(index.writeTree());
    repo.commit(c3_tree, 'c3', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Cherrypick c2 commit to feature branch
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });

    // Save original working directory state
    const filesBefore = await fs.readdir(p);
    const indexBefore = repo.index();
    // Cherrypick with dryRun option (should not modify working tree)
    repo.cherrypick(repo.getCommit(c2_oid), {
      checkoutOptions: {
        dryRun: true,
      },
    });

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    // Verify that working directory is unchanged (dryRun prevents changes)
    const filesAfter = await fs.readdir(p);
    expect(filesAfter.sort()).toEqual(filesBefore.sort());

    // Verify that file-a.txt does not exist in working directory
    await expect(fs.readFile(path.join(p, 'file-a.txt'), 'utf-8')).rejects.toThrow();

    // Verify that index has not been changed (dryRun only prevents changes)
    const indexAfter = repo.index();
    const diff = repo.diffIndexToIndex(indexBefore, indexAfter);
    const deltas = Array.from(diff.deltas());
    expect(deltas.length).toBe(0);

    // Clean up the state of the repository
    repo.cleanupState();
  });

  it('simple cherrypick_commit ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    /**
     * Create the following: (c2' is the cherrypick of c2)
     *
     * Base Commit Tree
     *   /---c3(feature)
     * c1---c2(main)
     *
     */

    // Commit c1 in main branch
    let index = repo.index();
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Commit c2 with file in main branch after c1
    index = repo.index();
    await fs.writeFile(path.join(p, 'c2_file'), 'c2_content');
    index.addPath('c2_file');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'c2', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Set head to detached state to avoid merge conflicts
    repo.setHeadDetached(repo.getCommit(c1_oid));
    repo.checkoutHead({ force: true });

    // Commit c3 in feature branch
    index = repo.index();
    const c3_tree = repo.getTree(index.writeTree());
    const c3_oid = repo.commit(c3_tree, 'c3', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Checkout feature branch
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });

    // Save original index  and working directory
    const indexBefore = repo.index();
    const filesBefore = await fs.readdir(p);

    // CherrypickCommit returns the index resulting from the cherrypick in memory, without affecting HEAD or the working tree.
    const cherrypickCommitIndex = repo.cherrypickCommit(repo.getCommit(c2_oid), repo.getCommit(c3_oid), 0);

    // Verify that the state of the repository is not "CherryPick", because cherrypickCommit does not modify the repository state.
    expect(repo.state()).not.toBe('CherryPick');

    // Verify: There should be no changes to the index or working directory (there should be no difference)
    const indexAfter = repo.index();
    const filesAfter = await fs.readdir(p);
    expect(indexAfter.writeTree()).toBe(indexBefore.writeTree());
    expect(filesAfter.sort()).toEqual(filesBefore.sort());

    // Cherrypick c2 commit to feature branch as c2'
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });

    // Cherrypick c2 commit to feature branch
    repo.cherrypick(repo.getCommit(c2_oid));

    // Verify that the state of the repository is "CherryPick"
    expect(repo.state()).toBe('CherryPick');

    // Index of the cherrypick commit
    const cherrypickIndex = repo.index();

    // Verify that cherrypick and cherrypick_commit result is the same index(compare isEmpty)
    expect(cherrypickIndex.isEmpty()).toBe(cherrypickCommitIndex.isEmpty());

    // Verify that cherrypick and cherrypick_commit result is the same index(compare diff)
    const diff = repo.diffIndexToIndex(cherrypickIndex, cherrypickCommitIndex);
    const deltas = Array.from(diff.deltas());
    expect(deltas.length).toBe(0);

    // Clean up the state of the repository
    repo.cleanupState();
  });

  it('cherrypickCommit with conflict ', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);

    /**
     * Create the following:
     *
     * Base Commit Tree
     *   /---c2(feature)
     * c1---c3(main)
     *
     * c2 and c3 both modify the same file, causing conflict when cherrypicking c2 to main
     */

    // Commit c1 in main branch
    let index = repo.index();
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nline2\nline3');
    index.addPath('conflict.txt');
    const c1_tree = repo.getTree(index.writeTree());
    const c1_oid = repo.commit(c1_tree, 'c1', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    // Commit c2 in feature branch (modify conflict.txt)
    repo.createBranch('feature', repo.getCommit(c1_oid));
    repo.setHead('refs/heads/feature');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nmodified2\nline3');
    index = repo.index();
    index.addPath('conflict.txt');
    const c2_tree = repo.getTree(index.writeTree());
    const c2_oid = repo.commit(c2_tree, 'c2', {
      updateRef: 'refs/heads/feature',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Commit c3 in main branch (modify conflict.txt differently)
    repo.setHead('refs/heads/main');
    repo.checkoutHead({ force: true });
    await fs.writeFile(path.join(p, 'conflict.txt'), 'line1\nchanged2\nline3');
    index = repo.index();
    index.addPath('conflict.txt');
    const c3_tree = repo.getTree(index.writeTree());
    const c3_oid = repo.commit(c3_tree, 'c3', {
      updateRef: 'refs/heads/main',
      author: signature,
      committer: signature,
      parents: [c1_oid],
    });

    // Save original working directory state
    const filesBefore = await fs.readdir(p);
    const contentBefore = await fs.readFile(path.join(p, 'conflict.txt'), 'utf-8');

    // CherrypickCommit with conflict (should not throw with failOnConflict: false)
    const cherrypickCommitIndex = repo.cherrypickCommit(repo.getCommit(c2_oid), repo.getCommit(c3_oid), 0, {
      failOnConflict: false,
    });

    // Verify that the state of the repository is not "CherryPick", because cherrypickCommit does not modify the repository state.
    expect(repo.state()).not.toBe('CherryPick');

    // Verify that conflicts exist in the returned index
    expect(cherrypickCommitIndex.hasConflicts()).toBe(true);

    // Verify: There should be no changes to the working directory (cherrypickCommit only returns index in memory)
    const filesAfter = await fs.readdir(p);
    const contentAfter = await fs.readFile(path.join(p, 'conflict.txt'), 'utf-8');
    expect(filesAfter.sort()).toEqual(filesBefore.sort());
    expect(contentAfter).toBe(contentBefore);
  });
});
