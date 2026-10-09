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

To create a signed commit and update the current branch, first use `commitCreateBuffer()` to prepare the unsigned content, sign it with GPG, and pass the signature to `commit()` with `updateRef: 'HEAD'`. This example requires GPG and a configured signing key.

```ts
import { openRepository } from 'es-git';
import { execFileSync } from 'node:child_process';

const repo = await openRepository('.');
const index = repo.index();
const treeOid = index.writeTree();
const tree = repo.getTree(treeOid);

const timeOptions = { timestamp: Math.floor(Date.now() / 1000), offset: 0 };
const author = { name: 'Seokju Na', email: 'seokju.me@toss.im', timeOptions };
const committer = { name: 'Seokju Na', email: 'seokju.me@toss.im', timeOptions };
const parents = [repo.head().target()!];
const message = 'signed commit';

const content = repo.commitCreateBuffer(tree, message, { author, committer, parents });
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');

const oid = repo.commit(tree, message, {
  author,
  committer,
  parents,
  signature,
  updateRef: 'HEAD',
});
```

`commit()` rebuilds the unsigned content before attaching the signature. Both calls must use the same tree, message, author, committer, and parents. Fix `timeOptions` for both the author and committer, as above: without explicit timestamps, each call samples the current time and can produce different bytes. Sign the exact UTF-8 bytes returned by `commitCreateBuffer()` without changing whitespace or line endings.

The signature is stored in Git's default `gpgsig` field. Both signed commit methods remove one trailing LF (`\n`) from the supplied signature, if present, so GPG's output does not add an extra blank line to the stored header. The commit content is unchanged. libgit2 does not cryptographically verify the signature; successfully writing or extracting it does not prove that it is valid.

### Writing only the commit object

To store the signed content directly, use `commitSigned()` instead of the `commit()` call above:

```ts
const oid = repo.commitSigned(content, signature);

const signatureInfo = repo.extractSignature(oid);
console.log(signatureInfo?.signedData === content); // true
```

`commitSigned()` writes the commit to the object database without updating `HEAD`, another reference, or a reflog. It uses `gpgsig` by default. An optional third argument selects a different signature field; the field name must be nonempty and contain no whitespace or NUL characters.
