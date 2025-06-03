# checkoutTree

Updates files in the index and working tree to match the content of the
tree pointed at by the treeish.

## Signature

```ts
class Repository {
  checkoutTree(treeish: GitObject, options?: CheckoutOptions | undefined | null): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">treeish</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object which tree pointed.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CheckoutOptions</span>
    <br>
    <p class="param-description">Options for checkout.</p>
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