import fs from 'node:fs/promises';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { type DiffDelta, type DiffFile, openRepository } from '../index';
import { isTarget } from './env';
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
    await fs.writeFile(path.join(p, 'new'), 'new created');
    const index = repo.index();
    index.addPath('new');
    index.write();
    const diff = repo.diffTreeToWorkdirWithIndex(repo.head().peelToTree());
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
        status: 'Added',
        oldFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'new',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
        newFile: {
          id: '706aecd4acb830a38b099ede1d9f4010bc9a0312',
          path: 'new',
          size: 11n,
          isBinary: false,
          isValidId: true,
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

  it('get diff include untracked', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    if (isTarget('win32')) {
      repo.config().setBool('core.autocrlf', true);
    }
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
    expect(deltas.length).toBe(expected.length);
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
    expect(diff.print({ format: 'PatchHeader' })).toEqual(`diff --git a/first b/first
index 9c59e24..46d3a3e 100644
--- a/first
+++ b/first
diff --git a/second b/second
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

  it('find renamed diff delta', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    if (isTarget('win32')) {
      repo.config().setBool('core.autocrlf', true);
    }
    const headTree = repo.head().peelToTree();
    await fs.rename(path.join(p, 'first'), path.join(p, 'first-renamed'));
    const index = repo.index();
    index.addPath('first-renamed');
    index.write();
    const diff = repo.diffTreeToWorkdirWithIndex(headTree);
    diff.findSimilar({ renames: true });
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 2,
        status: 'Renamed',
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
          id: '9c59e24b8393179a5d712de4f990178df5734d99',
          path: 'first-renamed',
          size: 6n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
      },
    ];
    expect(deltas.length).toBe(expected.length);
    expect(deltas.map(flattenDiffDelta)).toEqual(expect.arrayContaining(expected));
  });
});
