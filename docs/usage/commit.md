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

You can create GPG signed commits by providing a signature string:

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const signature = { name: 'Seokju Na', email: 'seokju.me@toss.im' };

// Create a signed commit
const oid = repo.commit(tree, 'signed commit', {
    updateRef: 'HEAD',
    author: signature,
    committer: signature,
    parents: [repo.head().target()!],
    signature: '-----BEGIN PGP SIGNATURE-----\\nVersion: GnuPG v1\\n\\niQEcBAABAgAGBQJTest123\\n-----END PGP SIGNATURE-----'
});

// Extract signature from the commit
const signatureInfo = repo.extractSignature(oid);
console.log(signatureInfo.signature);
// {
//  signature: '-----BEGIN PGP SIGNATURE-----\\nVersion: GnuPG v1\\n\\niQEcBAABAgAGBQJTest123\\n-----END PGP SIGNATURE-----',
//  signedData: 'tree ab9abf28de846b5968a8f12156f1d5ce3f4a198e\n' +
//  'parent a01e9888e46729ef4aa68953ba19b02a7a64eb82\n' +
//  'author Seokju Na <seokju.me@toss.im> 1744517729 +0000\n' +
//  'committer Seokju Na <seokju.me@toss.im> 1744517729 +0000\n' +
//  '\n' +
//  'signed commit'
// }
```
