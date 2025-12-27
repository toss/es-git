import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { openRepository, Repository } from '../index';
import { useFixture } from './fixtures';

describe('reflog', async () => {
  const signature = { name: 'racgoo', email: 'racgoo@example.com' };
  let repo: Repository;

  // Initialize repository and set user signature
  beforeEach(async () => {
    const p = await useFixture('empty');
    repo = await openRepository(p);
    const config = repo.config();
    config.setString('user.name', signature.name);
    config.setString('user.email', signature.email);
  });

  // Clean up repository after each test
  afterEach(async () => {
    repo.cleanupState();
  });

  it('get reflog', async () => {
    // Generate first commit(append reflog entry)
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('b1', repo.getCommit(oid));
    repo.createBranch('b2', repo.getCommit(oid));

    /// checkout branches(append reflog entry)
    repo.setHead('refs/heads/b1');
    repo.checkoutHead();
    repo.setHead('refs/heads/b2');
    repo.checkoutHead();
    repo.setHead('refs/heads/main');
    repo.checkoutHead();

    const reflog = repo.reflog('HEAD');

    expect(reflog).toBeDefined();
    // 4 reflog entries: 1 commit, 3 checkout
    expect(reflog.len()).toBe(4);
  });

  it('reflog entry properties', async () => {
    const commitMessage = 'test';
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), commitMessage, {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    const reflog = repo.reflog('HEAD');
    const entry = reflog.get(0);

    expect(entry).toBeDefined();

    // Type guard for entry, is not null
    if (entry) {
      const committer = entry.committer();

      expect(committer).toBeDefined();
      expect(committer.name).toBe(signature.name);
      expect(committer.email).toBe(signature.email);

      const idNew = entry.idNew();
      const idOld = entry.idOld();

      expect(idNew).toBeDefined();
      expect(idOld).toBeDefined();

      // Hex string with 40 characters
      expect(idNew).toMatch(/^[0-9a-f]{40}$/);
      expect(idOld).toMatch(/^[0-9a-f]{40}$/);

      // Oid(first commit) should be the same as the reflog entry idNew
      expect(idNew).toBe(oid);

      const message = entry.message();
      expect(message).toBeDefined();
      if (message) {
        expect(message.includes(commitMessage)).toBe(true);
      }

      const messageBytes = entry.messageBytes();
      expect(messageBytes).toBeDefined();
    }
  });

  it('reflog iterator', async () => {
    /**
     * Commit Shape
     *   c1(main)---c2(feature)
     */
    const oid1 = repo.commit(repo.getTree(repo.index().writeTree()), 'c1', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('feature', repo.getCommit(oid1));
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();

    const oid2 = repo.commit(repo.getTree(repo.index().writeTree()), 'c2', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [oid1],
    });

    const reflog = repo.reflog('HEAD');
    const entries: Array<{ idNew: string }> = [];

    for (const entry of reflog.iter()) {
      entries.unshift({
        idNew: entry.idNew(),
      });
    }

    expect(entries.length).toBe(3);

    const firstEntry = entries.at(0);
    if (firstEntry) {
      expect(firstEntry.idNew === oid1).toBe(true);
    }

    const middleEntry = entries.at(1);
    if (middleEntry) {
      expect(middleEntry.idNew === oid1).toBe(true);
    }

    const lastEntry = entries.at(2);
    if (lastEntry) {
      expect(lastEntry.idNew === oid2).toBe(true);
    }
  });

  it('reflog get with invalid index', async () => {
    repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    const reflog = repo.reflog('HEAD');
    const len = reflog.len();

    // Valid index
    const validEntry = reflog.get(0);
    expect(validEntry).toBeDefined();

    // Invalid index (out of bounds)
    const invalidEntry = reflog.get(len);
    expect(invalidEntry).toBeNull();
  });

  it('reflog append and write', async () => {
    repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    const reflog = repo.reflog('HEAD');
    const initialLen = reflog.len();

    // Get a signature from an existing entry
    const existingEntry = reflog.get(0);
    expect(existingEntry).toBeDefined();

    if (!existingEntry) return;
    const sig = existingEntry.committer();

    // Append new entry
    const newOid = repo.head().target()!;
    const newMessage = 'test message';
    reflog.append(newOid, sig, newMessage);
    reflog.write();

    // Verify entry was added
    const updatedReflog = repo.reflog('HEAD');
    expect(updatedReflog.len()).toBe(initialLen + 1);

    const newEntry = updatedReflog.get(0);
    expect(newEntry).toBeDefined();

    if (newEntry) {
      expect(newEntry.idNew()).toBe(newOid);
      expect(newEntry.message()).toBe(newMessage);
    }
  });

  it('reflog remove', async () => {
    const oid1 = repo.commit(repo.getTree(repo.index().writeTree()), 'c1', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('feature', repo.getCommit(oid1));
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();

    repo.commit(repo.getTree(repo.index().writeTree()), 'c2', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [oid1],
    });

    const reflog = repo.reflog('HEAD');
    const initialLen = reflog.len();
    expect(initialLen).toBe(3);

    // Remove first entry
    reflog.remove(0, false);
    reflog.write();

    // Verify entry was removed
    const updatedReflog = repo.reflog('HEAD');
    expect(updatedReflog.len()).toBe(initialLen - 1);
  });

  it('reflog remove with rewritePreviousEntry', async () => {
    // Commit 3 times
    for (let i = 0; i < 3; i++) {
      repo.commit(repo.getTree(repo.index().writeTree()), `c${i + 1}`, {
        updateRef: 'HEAD',
        author: signature,
        committer: signature,
        parents: [repo.head().target()!],
      });
    }

    const reflog = repo.reflog('HEAD');
    const initialLen = reflog.len();
    expect(initialLen).toBe(3);

    // Remove the c2 entry and rewrite the previous entry(c1's new oid should be c3's old oid)
    reflog.remove(1, true);
    reflog.write();

    const updatedReflog = repo.reflog('HEAD');

    expect(updatedReflog.len()).toBe(initialLen - 1);

    const c1Entry = updatedReflog.get(1);
    const c3Entry = updatedReflog.get(0);

    // Type guard for entries, is not null
    if (c1Entry && c3Entry) {
      expect(c1Entry.idNew()).toBe(c3Entry.idOld());
    }
  });

  it('reflog rename', async () => {
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('old', repo.getCommit(oid));

    repo.setHead('refs/heads/old');
    repo.checkoutHead();

    const oldReflog = repo.reflog('refs/heads/old');
    const initialLen = oldReflog.len();
    expect(initialLen).toBe(1);

    // Rename the reflog
    repo.reflogRename('refs/heads/old', 'refs/heads/new');

    // Verify new reflog exists and has the same entries
    const newReflog = repo.reflog('refs/heads/new');
    expect(newReflog).toBeDefined();
    expect(newReflog.len()).toBe(initialLen);
  });

  it('reflog rename with duplicate name', async () => {
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('feature-1', repo.getCommit(oid));
    repo.createBranch('feature-2', repo.getCommit(oid));

    repo.setHead('refs/heads/feature-2');
    repo.checkoutHead();

    // Commit 3 times in feature-2 branch, total 4 entries in feature-2 reflog
    for (let i = 0; i < 3; i++) {
      repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
        updateRef: 'HEAD',
        author: signature,
        committer: signature,
        parents: [repo.head().target()!],
      });
    }

    repo.setHead('refs/heads/feature-1');
    repo.checkoutHead();

    // Reflog(feature-1) should have 1 entry(create branch)
    const firstFeatureReflog = repo.reflog('refs/heads/feature-1');
    const firstFeatureInitialLen = firstFeatureReflog.len();
    expect(firstFeatureInitialLen).toBe(1);

    // Reflog(feature-2) should have 4 entries(commit 3 times)
    const secondFeatureReflog = repo.reflog('refs/heads/feature-2');
    const secondFeatureInitialLen = secondFeatureReflog.len();
    expect(secondFeatureInitialLen).toBe(4);

    // Rename the reflog with duplicate name (overwrite the reflog)
    repo.reflogRename('refs/heads/feature-1', 'refs/heads/feature-2');

    // Verify new reflog exists and has the same entries
    const overwrittenReflog = repo.reflog('refs/heads/feature-2');
    expect(overwrittenReflog).toBeDefined();
    expect(overwrittenReflog.len()).toBe(firstFeatureInitialLen);
  });

  it('reflog delete', async () => {
    const oid = repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    repo.createBranch('feature', repo.getCommit(oid));
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();

    // Delete the reflog(truncate the reflog)
    repo.reflogDelete('refs/heads/feature');

    // Verify the reflog is truncated
    const deletedReflog = repo.reflog('refs/heads/feature');
    expect(deletedReflog.len()).toBe(0);
  });

  it('reflog is_empty', async () => {
    const headCommit = repo.getCommit(repo.head().target()!);
    repo.createBranch('feature', headCommit);
    repo.setHead('refs/heads/feature');
    repo.checkoutHead();

    // Verify the reflog is empty
    const reflog = repo.reflog('refs/heads/feature');

    // Create entry is exists
    expect(reflog.isEmpty()).toBe(false);

    // Delete the reflog
    repo.reflogDelete('refs/heads/feature');

    // Verify the reflog is empty
    const deletedReflog = repo.reflog('refs/heads/feature');
    expect(deletedReflog.isEmpty()).toBe(true);
  });

  it('reflog append with null message', async () => {
    repo.commit(repo.getTree(repo.index().writeTree()), 'test', {
      updateRef: 'HEAD',
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
    });

    const reflog = repo.reflog('HEAD');
    const newOid = repo.head().target()!;

    // Get a signature from an existing entry
    const existingEntry = reflog.get(0);
    expect(existingEntry).toBeDefined();

    // Type guard for entry, is not null
    if (!existingEntry) return;
    const sig = existingEntry.committer();

    // Append entry without message
    reflog.append(newOid, sig, null);
    reflog.write();

    const updatedReflog = repo.reflog('HEAD');
    const entry = updatedReflog.get(0);
    expect(entry).toBeDefined();
    // Type guard for entry, is not null
    if (entry) {
      const message = entry.message();
      expect(message).toBeNull();
    }
  });
});
