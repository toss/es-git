# Diff

es-git supports a variety of methods for comparing diff. You can refer to each method’s documentation for detailed usage instructions.

- [`diffIndexToIndex()`](../reference/Repository/Methods/diffIndexToIndex.md) : Create a diff between two index objects.
- [`diffIndexToWorkdir()`](../reference/Repository/Methods/diffIndexToWorkdir.md) : Create a diff between the repository index and the working directory. (This matches the `git diff` command.)
- [`diffTreeToTree()`](../reference/Repository/Methods/diffTreeToTree.md) : Create a diff with the difference between two tree objects. (This is equivalent to `git diff <old-tree> <new-tree>` command.)
- [`diffTreeToWorkdir()`](../reference/Repository/Methods/diffTreeToWorkdir.md) : Create a diff between a tree and the working directory.
- [`diffTreeToWorkdirWithIndex()`](../reference/Repository/Methods/diffTreeToWorkdirWithIndex.md): Create a diff between a tree and the working directory using index data to account for staged deletes, tracked files, etc.

---

Below is an example demonstrating how to combine the diff between the `HEAD` tree and the index, and the diff between the index and the working directory using the [`diffTreeToWorkdirWithIndex()`](../reference/Repository/Methods/diffTreeToWorkdirWithIndex.md) method.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
await fs.writeFile('newFile', 'data', 'utf8');

const index = repo.index();
index.addPath('newFile');
index.write();

const headTree = repo.head().peelToTree();
const diff = repo.diffTreeToWorkdirWithIndex(headTree);
const deltas = [...diff.deltas()];

console.assert(deltas[0].status()); // "Added"
console.assert(deltas[0].newFile().path()); // "newfile"
```

## Comparing with a Remote Branch

Remote branches can be fetched using the [`fetch()`](../reference/Remote/Methods/fetch.md) method, allowing you to compare the trees of a remote branch with a local branch.

This can be useful for optimizing CI pipelines, such as when merging pull requests. For instance, in GitHub Actions, you can reference the [`github.base_ref`](https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions#github-context) value in a workflow triggered by a pull request event to obtain the reference of the remote branch for comparison.

The example below compares the trees between the remote `main` branch and the local `HEAD` reference to retrieve the changes.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const remote = repo.getRemote('origin');
await remote.fetch(['main'], {
  fetch: {
    depth: 1,
  },
});

const baseTree = repo.getReference('refs/remotes/origin/main').peelToTree();
const headTree = repo.head().peelToTree();

// Diff between the tree pointed to by the remote "origin/main" branch and the tree pointed to by the local "HEAD".
const diff = repo.diffTreeToTree(baseTree, headTree);

for (const delta of diff.deltas()) {
  // ...
}
```

## Displaying Formatted Text Output

Just as you can view text output from the `git diff` command, you can also use the [`print()`](../reference/Diff/Methods/print.md) method to output the changes as formatted text.

This allows you to quickly get an overview of the changes without needing to iterate over each delta individually, which can be very helpful for debugging.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const diff = repo.diffIndexToWorkdir();

console.log(diff.print());
/**
 * diff --git a/first b/first
 * index 9c59e24..46d3a3e 100644
 * --- a/first
 * +++ b/first
 * @@ -1 +1,2 @@
 * first
 * first modified
 *
 * diff --git a/second b/second
 * deleted file mode 100644
 * index e019be0..0000000
 * --- a/second
 * +++ /dev/null
 * @@ -1 +0,0 @@
 * second
 */
```

## Tracking Renames

Since git does not explicitly manage changes in file names or file movements, you need to enable the `renames` option to track files that have been renamed.

By using the [`findSimilar()`](../reference/Diff/Methods/findSimilar.md) method, you can configure various options while tracking changes.

In the example below, enabling the `renames` option demonstrates that files that have been renamed are marked with a `"Renamed"` status when comparing changes.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
const headTree = repo.head().peelToTree();

await fs.rename('myfile', 'myfile-renamed');
const index = repo.index();
index.addPath('myfile-renamed');
index.write();

const diff = repo.diffTreeToWorkdirWithIndex(headTree);
diff.findSimilar({ renames: true });

for (const delta of diff.deltas()) {
  if (detla.status() === 'Renamed') {
    const status = delta.status().toUpperCase();
    const oldPath = detla.oldFile().path();
    const newPath = detla.newFile().path();
    console.log(`[${status}] ${oldPath} -> ${newPath}`);
  }
}

/**
 * The outputs will be:
 * [RENAMED] myfile -> myfile-renamed
 */
```
