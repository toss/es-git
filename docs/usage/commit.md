# Commit Changes

Here's a simple example of how to commit changes. The code below creates a new commit on the currently active branch.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
 
await fs.writeFile('README.md', 'Hello World!', 'utf8');

const index = repo.index();
index.addPath('README.md');

const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const oid = repo.commit(tree, 'added new file', {
  updateRef: 'HEAD',
  author: signature,
  committer: signature,
  parents: [repo.head().target()!],
});

const commit = repo.getCommit(oid);
console.log(commit.summary()); // "added new file"
```

If you want to stage all files in the staging area, similar to the `git add *` command, you can use [`addAll()`](../reference/Index/Methods/addAll.md).

```ts
const index = repo.index();
index.addAll(['*']);
index.write();
```
