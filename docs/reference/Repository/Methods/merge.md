# merge

Merges the given commit(s) into HEAD, writing the results into the
working directory. Any changes are staged for commit and any conflicts
are written to the index. Callers should inspect the repository's index
after this completes, resolve any conflicts and prepare a commit.

For compatibility with git, the repository is put into a merging state.
Once the commit is done (or if the user wishes to abort), you should
clear this state by calling `cleanupState()`.

## Signature

```ts
class Repository {
  merge(
    annotatedCommits: AnnotatedCommit[],
    mergeOptions?: MergeOptions | undefined | null,
    checkoutOptions?: CheckoutOptions | undefined | null,
  ): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">annotatedCommits</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit[]</span>
    <br>
    <p class="param-description">Commits to merge.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mergeOptions</span><span class="param-type">null | MergeOptions</span>
    <br>
    <p class="param-description">Merge options.</p>
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
  <li class="param-li param-li-root">
    <span class="param-name">checkoutOptions</span><span class="param-type">null | CheckoutOptions</span>
    <br>
    <p class="param-description">Checkout options.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">In safe mode, apply safe file updates even when there are conflicts instead of canceling the checkout.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The name of the common ancestor side of conflicts</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicates whether to include common ancestor data in diff3 format files for conflicts.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether a normal merge file should be written for conflicts.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">dirPerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Set the mode with which new directories are created.  Default is 0755</p>
      </li>
      <li class="param-li">
        <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether to apply filters like CRLF conversion.</p>
      </li>
      <li class="param-li">
        <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Treat paths specified in <code>path</code> as exact file paths instead of as pathspecs.</p>
      </li>
      <li class="param-li">
        <span class="param-name">dryRun</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate that this checkout should perform a dry run by checking for conflicts but not make any actual changes.</p>
      </li>
      <li class="param-li">
        <span class="param-name">filePerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Set the mode with which new files are created.  The default is 0644 or 0755 as dictated by the blob.</p>
      </li>
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Take any action necessary to get the working directory to match the target including potentially discarding modified files.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ourLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The name of the common our side of conflicts</p>
      </li>
      <li class="param-li">
        <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether ignored files should be overwritten during the checkout.  Defaults to true.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Add a path to be checked out.  The path is a &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; pattern, unless <code>disablePathspecMatch</code> is set.  If no paths are specified, then all files are checked out. Otherwise only these specified paths are checked out.</p>
      </li>
      <li class="param-li">
        <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">In safe mode, create files that don&#39;t exist.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">refresh</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether the index and git attributes should be refreshed from disk before any operations.  Defaults to true,</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Remove ignored files from the working dir.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Remove untracked files from the working dir.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">safe</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate that the checkout should be performed safely, allowing new files to be created but not overwriting existing files or changes.  This is the default.</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Skip files with unmerged index entries.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetDir</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Set the directory to check out to</p>
      </li>
      <li class="param-li">
        <span class="param-name">theirLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The name of the common their side of conflicts</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Prevents checkout from writing the updated files&#39; information to the index.  Defaults to true.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Only update the contents of files that already exist.  If set, files will not be created or deleted.  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useOurs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether the checkout should proceed on conflicts by using the stage 2 version of the file (&quot;ours&quot;).  Defaults to false.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicate whether the checkout should proceed on conflicts by using the stage 3 version of the file (&quot;theirs&quot;).  Defaults to false.</p>
      </li>
    </ul>
  </li>
</ul>