import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it, vi } from 'vitest';
import { type IndexEntry, openRepository } from '../index';
import { useFixture } from './fixtures';

function entryPath(entry: IndexEntry): string {
  return entry.path.toString('utf8');
}

describe('index', () => {
  it('can get index of repository', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const index = repo.index();
    expect(index.count()).toBe(2);
    expect(index.isEmpty()).toBe(false);
    expect(index.path()).toMatch(path.join(p, '.git', 'index'));
  });

  it('get entries of index', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const index = repo.index();
    expect([...index.entries()].map(entryPath)).toEqual(['first', 'second']);
  });

  it('add with path', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const index = repo.index();
    await fs.writeFile(path.join(p, 'A'), 'A');
    index.addPath('A');
    expect([...index.entries()].map(entryPath)).toContain('A');
  });

  it('add all with pathspecs', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const index = repo.index();
    await fs.writeFile(path.join(p, 'A'), 'A');
    await fs.writeFile(path.join(p, 'B'), 'B');
    const onMatch = vi.fn().mockReturnValue(0);
    index.addAll(['*'], { onMatch });
    const entryPaths = [...index.entries()].map(entryPath);
    expect(entryPaths).toContain('A');
    expect(entryPaths).toEqual(expect.arrayContaining(['A', 'B']));
    expect(index.count()).toBe(2);
    expect(onMatch).toHaveBeenCalledTimes(2);
    const calls = onMatch.mock.calls;
    expect(calls).toEqual(expect.arrayContaining([[{ path: 'A', pathspec: '*' }], [{ path: 'B', pathspec: '*' }]]));
  });

  it('remove all with pathspecs', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const index = repo.index();
    await fs.writeFile(path.join(p, 'A'), 'A');
    index.addAll(['*']);
    expect(index.count()).toBe(1);
    index.removeAll(['A']);
    expect(index.count()).toBe(0);
  });

  it('update all with pathspecs', async () => {
    const p = await useFixture('empty');
    const repo = await openRepository(p);
    const index = repo.index();
    await fs.writeFile(path.join(p, 'A'), 'A');
    index.addAll(['*']);
    expect(index.getByPath('A')?.fileSize).toBe(1);
    await fs.writeFile(path.join(p, 'A'), 'AA');
    index.updateAll(['*']);
    expect(index.getByPath('A')?.fileSize).toBe(2);
  });
});
