# Commit Changes

Here's a simple example of how to commit changes. The code below creates a new commit on the currently active branch.

```ts
import { openRepository } from 'es-git';
import fs from 'node:fs/promises';

const repo = await openRepository('.');
 
await fs.writeFile('README.md', 'Hello World!', 'utf8');

const index = repo.index();
index.addPath('README.md');
index.write();

const tree = repo.head().peelToTree();
const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const sha = repo.commit(tree, 'added new file', {
  author: signature,
  committer: signature,
});

const commit = repo.getCommit(sha);
console.log(commit.summary()); // "added new file"
```

If you want to stage all files in the staging area, similar to the `git add *` command, you can use [`addAll()`](../reference/classes/Index.md#addall).

```ts
const index = repo.index();
index.addAll(['*']);
index.write();
```
