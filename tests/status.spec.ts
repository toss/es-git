import { writeFile } from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('status', () => {
  it('get status entries', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await writeFile(path.join(p, 'added'), 'added', 'utf8');
    await writeFile(path.join(p, 'first'), 'first-modified', 'utf8');
    await writeFile(path.join(p, 'added-to-index'), 'added-to-index', 'utf8');

    const index = repo.index();
    index.addPath('added-to-index');
    index.write();

    const statuses = repo.statuses();
    const entries = [...statuses.iter()];
    expect(entries).toHaveLength(3);
    const getEntry = (path: string) => entries.find(x => x.path() === path)!;
    expect(getEntry('added').status()).toEqual(
      expect.objectContaining({
        current: true,
        indexNew: false,
        wtNew: true,
      })
    );
    expect(getEntry('added-to-index').status()).toEqual(
      expect.objectContaining({
        current: true,
        indexNew: true,
        wtNew: false,
      })
    );
    expect(getEntry('first').status()).toEqual(
      expect.objectContaining({
        current: true,
        indexNew: false,
        wtNew: false,
        wtModified: true,
      })
    );
  });
});
