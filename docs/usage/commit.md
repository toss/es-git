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

## Creating Signed Commits

External signing is a two-step process. First, create the unsigned commit content and pass its exact UTF-8 bytes to your signing backend. Then, write the signed commit with the returned signature.

Do not normalize whitespace or line endings in the content returned by `commitCreateBuffer()`. `commitSigned()` writes the commit to the object database, but it does not update `HEAD` or another reference.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
const commitContent = repo.commitCreateBuffer(tree, 'signed commit', {
  author: signature,
  committer: signature,
  parents: [repo.head().target()!],
});

// Implement this call with your GPG or other Git-compatible signing backend.
const externalSignature = await signCommitContent(Buffer.from(commitContent, 'utf8'));
const oid = repo.commitSigned(commitContent, externalSignature);

const signatureInfo = repo.extractSignature(oid);
console.log(signatureInfo?.signature === externalSignature); // true
console.log(signatureInfo?.signedData === commitContent); // true
```

If you already have a signature for the exact commit content that `commit()` will create, you can continue to pass it through `CommitOptions.signature`.
