import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { type DiffDelta, type DiffFile, openRepository } from '../index';
import { WIN32 } from './env';
import { useFixture } from './fixtures';
import type { FlattenMethods } from './types';

function flattenDiffFile(file: DiffFile): FlattenMethods<DiffFile> {
  return {
    id: file.id(),
    path: file.path(),
    size: file.size(),
    isBinary: file.isBinary(),
    isValidId: file.isValidId(),
    exists: file.exists(),
    mode: file.mode(),
  };
}

function flattenDiffDelta(delta: DiffDelta): FlattenMethods<DiffDelta> {
  return {
    flags: delta.flags(),
    numFiles: delta.numFiles(),
    status: delta.status(),
    oldFile: flattenDiffFile(delta.oldFile()),
    newFile: flattenDiffFile(delta.newFile()),
  };
}

describe('diff', () => {
  it('get diff', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'first'), 'first modified');
    await fs.rm(path.join(p, 'second'));
    const diff = repo.diffIndexToWorkdir();
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 2,
        status: 'Modified',
        oldFile: {
          id: '9c59e24b8393179a5d712de4f990178df5734d99',
          path: 'first',
          size: 6n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'first',
          size: 14n,
          isBinary: false,
          isValidId: false,
          exists: true,
          mode: 'Blob',
        },
      },
      {
        flags: 0,
        numFiles: 1,
        status: 'Deleted',
        oldFile: {
          id: 'e019be006cf33489e2d0177a3837a2384eddebc5',
          path: 'second',
          size: 7n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'second',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
      },
    ];
    expect(deltas).toHaveLength(expected.length);
    expect(deltas.map(flattenDiffDelta)).toEqual(expect.arrayContaining(expected));
  });

  // Windows track all files when 'includeUntracked' option is enabled.
  // Need to look further into why.
  it('get diff include untracked', { skip: WIN32 }, async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'third'), 'third created');
    const diff = repo.diffIndexToWorkdir(undefined, {
      includeUntracked: true,
    });
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 1,
        status: 'Untracked',
        oldFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'third',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'third',
          size: 13n,
          isBinary: false,
          isValidId: false,
          exists: true,
          mode: 'Blob',
        },
      },
    ];
    expect(deltas.length).toBe(1);
    expect(deltas.map(flattenDiffDelta)).toEqual(expect.arrayContaining(expected));
  });

  it('print diff with formatting', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'first'), 'first modified\n\n');
    await fs.rm(path.join(p, 'second'));
    const diff = repo.diffIndexToWorkdir(undefined);
    expect(diff.print()).toEqual(`diff --git a/first b/first
index 9c59e24..46d3a3e 100644
--- a/first
+++ b/first
@@ -1 +1,2 @@
first
first modified

diff --git a/second b/second
deleted file mode 100644
index e019be0..0000000
--- a/second
+++ /dev/null
@@ -1 +0,0 @@
second
`);
    expect(diff.print({ format: 'PatchHeader' })).toEqual(`diff --git a/second b/second
deleted file mode 100644
index e019be0..0000000
--- a/second
+++ /dev/null
`);
    expect(diff.print({ format: 'NameOnly' })).toEqual(`first
second
`);
    expect(diff.print({ format: 'Raw' })).toEqual(`:100644 100644 9c59e24... 46d3a3e... M\tfirst
:100644 000000 e019be0... 0000000... D\tsecond
`);
  });
});
