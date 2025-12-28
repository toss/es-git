# cherrypickCommit

Applies a cherrypick of `cherrypick_commit` against `our_commit` and returns the resulting Index,
without modifying the working directory or repository state.
This method does not write any changes to disk or update HEAD.
it is useful for computing what the cherrypick result would look like without actually applying it.

## Signature

```ts
class Repository {
  cherrypickCommit(
    cherrypickCommit: Commit,
    ourCommit: Commit,
    mainline: number,
    mergeOptions?: MergeOptions | undefined | null,
  ): Index;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">cherrypickCommit</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">The commit to cherrypick.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ourCommit</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">The commit to cherrypick against (usually HEAD).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mainline</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">The parent of the cherrypick commit, if it is a merge (1-based).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mergeOptions</span><span class="param-type">null | MergeOptions</span>
    <br>
    <p class="param-description">Options for merge conflict resolution.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Create diff3-style file</p>
      </li>
      <li class="param-li">
        <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If a conflict occurs, exit immediately instead of attempting to continue resolving conflicts</p>
      </li>
      <li class="param-li">
        <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
        <br>
        <p class="param-description">Specify a side to favor for resolving conflicts</p>
      </li>
      <li class="param-li">
        <span class="param-name">findRenames</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Detect file renames</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore all whitespace</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore changes in amount of whitespace</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore whitespace at end of line</p>
      </li>
      <li class="param-li">
        <span class="param-name">minimal</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Take extra time to find minimal diff</p>
      </li>
      <li class="param-li">
        <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If the commits being merged have multiple merge bases, do not build a recursive merge base (by merging the multiple merge bases), instead simply use the first base.</p>
      </li>
      <li class="param-li">
        <span class="param-name">patience</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Use the &quot;patience diff&quot; algorithm</p>
      </li>
      <li class="param-li">
        <span class="param-name">recursionLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Maximum number of times to merge common ancestors to build a virtual merge base when faced with criss-cross merges.  When this limit is reached, the next ancestor will simply be used instead of attempting to merge it.  The default is unlimited.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Similarity to consider a file renamed (default 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Condense non-alphanumeric regions for simplified diff file</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Do not write the REUC extension on the generated index</p>
      </li>
      <li class="param-li">
        <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Create standard conflicted merge files</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Maximum similarity sources to examine for renames (default 200). If the number of rename candidates (add / delete pairs) is greater than this value, inexact rename detection is aborted. This setting overrides the <code>merge.renameLimit</code> configuration value.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">The index result.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the cherrypick commit is a merge and mainline is 0.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If there are conflicts and failOnConflict is true (default).</p>
  </li>
</ul>

## Examples

```ts
// This is an example for cherrypick_commit
import { openRepository } from "es-git";

const repo = await openRepository("./path/to/repo");
const cherry = repo.getCommit("cherrypick-commit");
const target = repo.getCommit("onto-commit");

// Returns the Index resulting from the cherrypick in memory,
// without affecting HEAD or the working tree.
// The mainline parameter indicates which parent to use as the baseline,
// For merge commits, mainline specifies which parent to use as baseline (1 or 2).
// For normal (non-merge) commits, use mainline 0.
const idx = repo.cherrypickCommit(cherry, target, 0);

// You can check for conflicts with idx.hasConflicts()
```