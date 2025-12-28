import fs from 'node:fs/promises';
import path from 'node:path';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { Repository, Worktree, openRepository } from '../index';
import { useFixture } from './fixtures';
import { makeTmpDir } from './tmp';

describe('worktree', () => {
  const signature = { name: 'racgoo', email: 'racgoo@example.com' };
  let repo: Repository;
  let baseWorktreePath: string;

  beforeEach(async () => {
    const p = await useFixture('empty');
    repo = await openRepository(p);
    const config = repo.config();
    config.setString('user.name', signature.name);
    config.setString('user.email', signature.email);
    // Create initial commit
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), 'initial commit', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });
    // Create a branch for worktree
    repo.createBranch('worktree-branch', repo.getCommit(oid));
    // Create base temporary directory for worktrees
    baseWorktreePath = await makeTmpDir('worktree-test');
  });

  afterEach(async () => {
    // Clean up worktrees
    const worktrees = repo.worktrees();
    for (const name of worktrees) {
      repo.findWorktree(name).prune({ valid: true, locked: true, workingTree: true });
    }
    // Remove base worktree directory
    await fs.rm(baseWorktreePath, { recursive: true, force: true });
    repo.cleanupState();
  });

  it('add worktree', async () => {
    // Generate worktree
    const worktreeName = 'test';
    const worktreePath = path.join(baseWorktreePath, 'test-dir');
    const worktree = repo.worktree(worktreeName, worktreePath);
    expect(worktree).toBeDefined();

    // Open test-worktree repository (path based)
    const originRepoHeadName = repo.head().name();
    const worktreeRepo = Repository.openFromWorktree(worktree);
    const head = worktreeRepo.head();
    expect(head.name()).include(worktreeName);
    expect(head.name()).not.toBe(originRepoHeadName);
  });

  it('add worktree with options', async () => {
    const worktreeName = 'test';
    const worktreePath = path.join(baseWorktreePath, 'test-dir');
    const worktree = repo.worktree(worktreeName, worktreePath, {
      lock: true,
      checkoutExisting: false,
      refName: 'refs/heads/worktree-branch',
    });
    expect(worktree).toBeDefined();
    expect(worktree.name()).toBe(worktreeName);

    const lockStatus = worktree.isLocked();
    expect(lockStatus).toBeDefined();
    expect(lockStatus.status).toBe('Locked');

    // Open test-worktree repository(reference based)
    const worktreeRepo = Repository.openFromWorktree(worktree);
    const head = worktreeRepo.head();
    expect(head.name()).toBe('refs/heads/worktree-branch');
  });

  it('list worktrees', async () => {
    const worktree1 = repo.worktree('worktree1', path.join(baseWorktreePath, 'wt1-dir'));
    const worktree2 = repo.worktree('worktree2', path.join(baseWorktreePath, 'wt2-dir'));

    const worktrees = repo.worktrees();
    expect(worktrees).toContain('worktree1');
    expect(worktrees).toContain('worktree2');
  });

  it('get worktree name and path', async () => {
    const worktreeName = 'test';
    const worktreePath = path.join(baseWorktreePath, 'test-dir');

    const worktree = repo.worktree(worktreeName, worktreePath);
    expect(worktree.name()).toBe(worktreeName);

    // Normalize paths to handle symlink differences on macOS
    expect(await fs.realpath(worktree.path())).toBe(await fs.realpath(worktreePath));
  });

  it('lock and unlock worktree', async () => {
    const worktreeName = 'test';
    const worktreePath = path.join(baseWorktreePath, 'test-dir');
    const worktree = repo.worktree(worktreeName, worktreePath);

    // Initially unlocked
    let lockStatus = worktree.isLocked();
    expect(lockStatus.status).toBe('Unlocked');

    // Lock with reason
    const lockReason = 'test reason';
    worktree.lock(lockReason);
    lockStatus = worktree.isLocked();
    expect(lockStatus.status).toBe('Locked');
    expect(lockStatus.reason).toBe(lockReason);

    // Unlock
    worktree.unlock();
    lockStatus = worktree.isLocked();
    expect(lockStatus.status).toBe('Unlocked');
  });

  it('lock worktree without reason', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-lock-null');
    const worktree = repo.worktree('test-worktree', worktreePath);
    worktree.lock(null);
    const lockStatus = worktree.isLocked();
    expect(lockStatus.status).toBe('Locked');
    expect(lockStatus.reason).toBeUndefined();
  });

  it('validate worktree', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-validate');
    const worktree = repo.worktree('test-worktree', worktreePath);
    expect(() => worktree.validate()).not.toThrow();
  });

  it('find worktree by name', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-find');
    repo.worktree('test-worktree', worktreePath);
    const found = repo.findWorktree('test-worktree');
    expect(found).toBeDefined();
    expect(found.name()).toBe('test-worktree');
    // Normalize paths to handle symlink differences on macOS
    expect(await fs.realpath(found.path())).toBe(await fs.realpath(worktreePath));
  });

  it('find worktree throws error if not found', async () => {
    expect(() => repo.findWorktree('non-existent-worktree')).toThrow();
  });

  it('open repository from worktree', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-open-repo');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const worktreeRepo = Repository.openFromWorktree(worktree);
    expect(worktreeRepo).toBeDefined();
    expect(worktreeRepo.isWorktree()).toBe(true);
  });

  it('open worktree from repository', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-open-wt');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const worktreeRepo = Repository.openFromWorktree(worktree);
    const openedWorktree = Worktree.openFromRepository(worktreeRepo);
    expect(openedWorktree).toBeDefined();
    expect(openedWorktree.name()).toBe('test-worktree');
  });

  it('check if repository is worktree', async () => {
    expect(repo.isWorktree()).toBe(false);

    const worktreePath = path.join(baseWorktreePath, 'test-worktree-is-wt');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const worktreeRepo = Repository.openFromWorktree(worktree);
    expect(worktreeRepo.isWorktree()).toBe(true);
  });

  it('check if worktree is prunable', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-prunable');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const isPrunable = worktree.isPrunable();
    expect(typeof isPrunable).toBe('boolean');
  });

  it('check if worktree is prunable with options', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-prunable-opts');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const isPrunable = worktree.isPrunable({
      valid: true,
      locked: false,
      workingTree: false,
    });
    expect(typeof isPrunable).toBe('boolean');
  });

  it('prune worktree', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-prune');
    const worktree = repo.worktree('test-worktree', worktreePath);
    const worktreeName = worktree.name();
    expect(worktreeName).toBe('test-worktree');

    // Prune the worktree
    worktree.prune({
      valid: true,
      locked: false,
      workingTree: true,
    });

    // Worktree should be removed from list
    const worktrees = repo.worktrees();
    expect(worktrees).not.toContain('test-worktree');
  });

  it('add worktree with checkoutExisting option', async () => {
    // Create a branch with the same name as worktree
    const headOid = repo.head().target()!;
    repo.createBranch('existing-branch', repo.getCommit(headOid));
    const commitOid = repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'refs/heads/existing-branch',
      author: signature,
      committer: signature,
      parents: [headOid],
    });
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-checkout-existing');
    const worktree = repo.worktree('existing-branch', worktreePath, {
      checkoutExisting: true,
    });
    // Check worktree
    expect(worktree).toBeDefined();
    expect(worktree.name()).toBe('existing-branch');
    // Open worktree repository and check head
    const worktreeRepo = Repository.openFromWorktree(worktree);
    const head = worktreeRepo.head();
    expect(head.name()).toBe('refs/heads/existing-branch');
    expect(head.target()).toBe(commitOid);
  });

  it('add worktree with refName option', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-refname');
    const worktree = repo.worktree('test-worktree', worktreePath, {
      refName: 'refs/heads/worktree-branch',
    });
    expect(worktree).toBeDefined();

    const worktreeRepo = Repository.openFromWorktree(worktree);
    const head = worktreeRepo.head();
    expect(head.name()).toBe('refs/heads/worktree-branch');
  });

  it('add worktree throws error if path already exists', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-exists');
    // Create a file at the path
    await fs.writeFile(worktreePath, 'test');

    expect(() => repo.worktree('test-worktree', worktreePath)).toThrow();
  });

  it('add worktree throws error if invalid reference name', async () => {
    const worktreePath = path.join(baseWorktreePath, 'test-worktree-invalid-ref');
    expect(() =>
      repo.worktree('test-worktree', worktreePath, {
        refName: 'refs/heads/non-existent',
      })
    ).toThrow();
  });
});
