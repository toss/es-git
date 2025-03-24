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
    const p = await useFixture('diff');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'A'), 'A modified');
    await fs.rm(path.join(p, 'B'));
    await fs.writeFile(path.join(p, 'D'), 'D created');
    const index = repo.index();
    index.addPath('D');
    index.write();
    const diff = repo.diffTreeToWorkdirWithIndex(repo.head().peelToTree(), {
      ignoreWhitespaceEol: isTarget('win32'),
    });
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 2,
        status: 'Modified',
        oldFile: {
          id: 'f70f10e4db19068f79bc43844b49f3eece45c4e8',
          path: 'A',
          size: 2n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'A',
          size: 10n,
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
          id: '223b7836fb19fdf64ba2d3cd6173c6a283141f78',
          path: 'B',
          size: 2n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'B',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
      },
      {
        flags: 0,
        numFiles: 1,
        status: 'Added',
        oldFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'D',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
        newFile: {
          id: '99f203cbf201fb268d8c2e5a695033551383cb53',
          path: 'D',
          size: 9n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
      },
    ];
    expect(deltas).toHaveLength(expected.length);
    expect(deltas.map(flattenDiffDelta)).toEqual(expect.arrayContaining(expected));
  });

  it('get diff include untracked', async () => {
    const p = await useFixture('diff');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'D'), 'D');
    const diff = repo.diffIndexToWorkdir(undefined, {
      includeUntracked: true,
      ignoreWhitespaceEol: isTarget('win32'),
    });
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 1,
        status: 'Untracked',
        oldFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'D',
          size: 0n,
          isBinary: false,
          isValidId: true,
          exists: false,
          mode: 'Unreadable',
        },
        newFile: {
          id: '0000000000000000000000000000000000000000',
          path: 'D',
          size: 1n,
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
    const p = await useFixture('diff');
    const repo = await openRepository(p);
    await fs.writeFile(path.join(p, 'A'), 'A modified');
    await fs.rm(path.join(p, 'B'));
    const diff = repo.diffIndexToWorkdir(undefined, {
      ignoreWhitespaceEol: isTarget('win32'),
    });
    expect(diff.print()).toEqual(`diff --git a/A b/A
index f70f10e..784f93d 100644
--- a/A
+++ b/A
@@ -1 +1 @@
A
A modified
\\ No newline at end of file
diff --git a/B b/B
deleted file mode 100644
index 223b783..0000000
--- a/B
+++ /dev/null
@@ -1 +0,0 @@
B
`);
    expect(diff.print({ format: 'PatchHeader' })).toEqual(`diff --git a/A b/A
index f70f10e..784f93d 100644
--- a/A
+++ b/A
diff --git a/B b/B
deleted file mode 100644
index 223b783..0000000
--- a/B
+++ /dev/null
`);
    expect(diff.print({ format: 'NameOnly' })).toEqual(`A
B
`);
    expect(diff.print({ format: 'Raw' })).toEqual(`:100644 100644 f70f10e... 784f93d... M\tA
:100644 000000 223b783... 0000000... D\tB
`);
  });

  it('find renamed diff delta', async () => {
    const p = await useFixture('diff');
    const repo = await openRepository(p);
    const headTree = repo.head().peelToTree();
    await fs.rename(path.join(p, 'A'), path.join(p, 'A-renamed'));
    const index = repo.index();
    index.addPath('A-renamed');
    index.write();
    const diff = repo.diffTreeToWorkdirWithIndex(headTree, {
      ignoreWhitespaceEol: isTarget('win32'),
    });
    diff.findSimilar({ renames: true });
    const deltas = [...diff.deltas()];
    const expected: FlattenMethods<DiffDelta>[] = [
      {
        flags: 0,
        numFiles: 2,
        status: 'Renamed',
        oldFile: {
          id: 'f70f10e4db19068f79bc43844b49f3eece45c4e8',
          path: 'A',
          size: 2n,
          isBinary: false,
          isValidId: true,
          exists: true,
          mode: 'Blob',
        },
        newFile: {
          id: 'f70f10e4db19068f79bc43844b49f3eece45c4e8',
          path: 'A-renamed',
          size: 2n,
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
