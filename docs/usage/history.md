# Commit History

To retrieve the commit history, you can use the [`revwalk`](../api/classes/Repository.md#revwalk) to create a [`Revwalk`](../api/classes/Revwalk.md).

Since [`Revwalk`](../api/classes/Revwalk.md) implements the [Iteration Protocol](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols), you can easily iterate over the
commit history using the [`for...of`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of) statement.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');

/**
 * For example, assume the commit history of this repository looks like this:
 * ❯ git log --pretty=format:'%s %H %an <%ae>'
 * E 2b2278987ac96270325a2281499cced99a2a41c0 Seokju Na <seokju.me@toss.im>
 * D 2091c70eaf9d0b36c8a063d4eaa75e522b31fc6b Seokju Na <seokju.me@toss.im>
 * C 0ed417bb37612320099ac013cdf2b0c2614a7afc Seokju Na <seokju.me@toss.im>
 * B 8761d9d89b1198d913bb31fede8c45405fa16ef3 Seokju Na <seokju.me@toss.im>
 * A 38b79dc02462eedc463fa3028ed39ac4d0339608 Seokju Na <seokju.me@toss.im>
 */

// Start reading commits from the commit pointed to by HEAD.
const revwalk = repo.revwalk().pushHead();

// Iterate over Revwalk and read commit hashes.
// The code below produces results identical to the `git log` command shown above.
for (const sha of revwalk) {
  const commit = repo.getCommit(sha);
  const summary = commit.summary();
  const id = commit.id();
  const author = commit.author();
  console.log(`${summary} ${id} ${author.name} <${author.email}>`);
}
```

If you'd like to set a range of commits to retrieve, you can use the [`pushRange`](../api/classes/Revwalk.md#pushrange).

You can define the range in `<commit>..<commit>` format. The traversal begins from the right-hand commit, and commits after (and including) the left-hand commit are excluded from the traversal range.

```ts
const revwalk = repo.revwalk().pushRange('8761d9d..2b22789');

for (const sha of revwalk) {
  const commit = repo.getCommit(sha);
  const summary = commit.summary();
  const id = commit.id();
  const author = commit.author();
  console.log(`${summary} ${id} ${author.name} <${author.email}>`);
}

/**
 * The above code would produce the following output:
 * E 2b2278987ac96270325a2281499cced99a2a41c0 Seokju Na <seokju.me@toss.im>
 * D 2091c70eaf9d0b36c8a063d4eaa75e522b31fc6b Seokju Na <seokju.me@toss.im>
 * C 0ed417bb37612320099ac013cdf2b0c2614a7afc Seokju Na <seokju.me@toss.im>
 */
```
