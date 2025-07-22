import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('stash', () => {
  const signature = { name: 'Seokju Na', email: 'seokju.me@gmail.com' };

  it('save stash', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'Stashed content');
    const oid = repo.stashSave({ stasher: signature, message: 'WIP: test stash' });
    expect(oid).toBeDefined();
    expect(oid.toString()).toMatch(/^[0-9a-f]{40}$/);

    const content = await fs.readFile(path.join(p, 'second'), 'utf8');
    expect(content.trim()).toBe('second');
  });

  it('list stashes', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'First stash content');
    repo.stashSave({ stasher: signature, message: 'First stash' });

    await fs.writeFile(path.join(p, 'second'), 'Second stash content');
    repo.stashSave({ stasher: signature, message: 'Second stash' });

    const stashList = repo.stashList();
    expect(stashList.len()).toBe(2);

    const first = stashList.get(0);
    expect(first).toBeDefined();
    if (first) {
      expect(first.index()).toBe(0);
      expect(first.id().toString()).toMatch(/^[0-9a-f]{40}$/);
      expect(first.message()).toContain('Second stash');
    }

    const second = stashList.get(1);
    expect(second).toBeDefined();
    if (second) {
      expect(second.index()).toBe(1);
      expect(second.message()).toContain('First stash');
    }
  });

  it('iterate over stashes', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'First stash');
    repo.stashSave({ stasher: signature });

    await fs.writeFile(path.join(p, 'second'), 'Second stash');
    repo.stashSave({ stasher: signature });

    const stashList = repo.stashList();
    const entries = Array.from(stashList.iter());
    expect(entries.length).toBe(2);
    expect(entries[0]!.index()).toBe(0);
    expect(entries[1]!.index()).toBe(1);
  });

  it('apply stash', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    const originalContent = await fs.readFile(path.join(p, 'second'), 'utf8');
    const stashedContent = 'Stashed content for apply test';
    await fs.writeFile(path.join(p, 'second'), stashedContent);
    repo.stashSave({ stasher: signature });

    let content = await fs.readFile(path.join(p, 'second'), 'utf8');
    expect(content).toBe(originalContent);

    repo.stashApply(0);

    content = await fs.readFile(path.join(p, 'second'), 'utf8');
    expect(content).toBe(stashedContent);

    const stashList = repo.stashList();
    expect(stashList.len()).toBe(1);
  });

  it('drop stash', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'First stash');
    repo.stashSave({ stasher: signature });

    await fs.writeFile(path.join(p, 'second'), 'Second stash');
    repo.stashSave({ stasher: signature });

    let stashList = repo.stashList();
    expect(stashList.len()).toBe(2);

    repo.stashDrop(0);

    stashList = repo.stashList();
    expect(stashList.len()).toBe(1);
  });

  it('pop stash', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    const stashedContent = 'Stashed content for pop test';
    await fs.writeFile(path.join(p, 'second'), stashedContent);
    repo.stashSave({ stasher: signature });

    let stashList = repo.stashList();
    expect(stashList.len()).toBe(1);

    repo.stashPop(0);

    const content = await fs.readFile(path.join(p, 'second'), 'utf8');
    expect(content).toBe(stashedContent);

    stashList = repo.stashList();
    expect(stashList.len()).toBe(0);
  });

  it('stash with untracked files', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'untracked.txt'), 'Untracked content');
    repo.stashSave({
      stasher: signature,
      message: 'Stash with untracked',
      includeUntracked: true,
    });

    const exists = await fs
      .access(path.join(p, 'untracked.txt'))
      .then(() => true)
      .catch(() => false);
    expect(exists).toBe(false);

    repo.stashApply(0);

    const content = await fs.readFile(path.join(p, 'untracked.txt'), 'utf8');
    expect(content).toBe('Untracked content');
  });

  it('stash with keep index', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'test.txt'), 'original');
    let index = repo.index();
    index.addPath('test.txt');
    index.write();
    const treeSha = index.writeTree();
    const tree = repo.getTree(treeSha);
    repo.commit(tree, 'Add test.txt', {
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
      updateRef: 'HEAD',
    });

    await fs.writeFile(path.join(p, 'test.txt'), 'Staged content');
    index = repo.index();
    index.addPath('test.txt');
    index.write();

    await fs.writeFile(path.join(p, 'test.txt'), 'Staged content\nUnstaged changes');

    repo.stashSave({
      stasher: signature,
      keepIndex: true,
    });

    const content = await fs.readFile(path.join(p, 'test.txt'), 'utf8');
    expect(content).toBe('Staged content');

    const status = repo.statuses();
    expect(status.isEmpty()).toBe(false);
  });

  it('empty stash list', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    const stashList = repo.stashList();
    expect(stashList.len()).toBe(0);
    expect(stashList.isEmpty()).toBe(true);
    expect(stashList.get(0)).toBeNull();
  });

  it('stash apply with reinstantiate index', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'Staged changes');
    const index = repo.index();
    index.addPath('second');
    index.write();

    repo.stashSave({ stasher: signature });

    repo.stashApply(0, { reinstantiateIndex: true });

    const statusFile = repo.getStatusFile('second');
    expect(statusFile.indexModified).toBe(true);
  });

  it('stash without message', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, 'second'), 'Modified content');
    const oid = repo.stashSave({ stasher: signature });
    expect(oid).toBeDefined();
    expect(oid.toString()).toMatch(/^[0-9a-f]{40}$/);

    const stashList = repo.stashList();
    const entry = stashList.get(0);
    expect(entry).toBeDefined();
    if (entry) {
      const message = entry.message();
      expect(message).toBeTruthy(); // Should have a default message
    }
  });

  it('stash with ignored files', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    await fs.writeFile(path.join(p, '.gitignore'), '*.log\n');

    const index = repo.index();
    index.addPath('.gitignore');
    index.write();
    const treeSha = index.writeTree();
    const tree = repo.getTree(treeSha);
    repo.commit(tree, 'Add .gitignore', {
      author: signature,
      committer: signature,
      parents: [repo.head().target()!],
      updateRef: 'HEAD',
    });

    await fs.writeFile(path.join(p, 'test.log'), 'Ignored content');

    await fs.writeFile(path.join(p, 'normal.txt'), 'Normal content');

    repo.stashSave({
      stasher: signature,
      message: 'Stash with ignored',
      includeIgnored: true,
      includeUntracked: true,
    });

    const logExists = await fs
      .access(path.join(p, 'test.log'))
      .then(() => true)
      .catch(() => false);
    const normalExists = await fs
      .access(path.join(p, 'normal.txt'))
      .then(() => true)
      .catch(() => false);

    expect(logExists).toBe(false);
    expect(normalExists).toBe(false);

    repo.stashApply(0);

    const logContent = await fs.readFile(path.join(p, 'test.log'), 'utf8');
    expect(logContent).toBe('Ignored content');

    const normalContent = await fs.readFile(path.join(p, 'normal.txt'), 'utf8');
    expect(normalContent).toBe('Normal content');
  });

  it('error handling - invalid stash index', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    expect(() => repo.stashApply(999)).toThrow();
    expect(() => repo.stashDrop(999)).toThrow();
    expect(() => repo.stashPop(999)).toThrow();
  });

  it('error handling - get invalid index', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);

    const stashList = repo.stashList();
    expect(stashList.get(999)).toBeNull();
  });
});
