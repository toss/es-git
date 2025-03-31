# diffIndexToIndex

Create a diff between two index objects.

## Signature

```ts
class Repository {
  diffIndexToIndex(
    oldIndex: Index,
    newIndex: Index,
    options?: DiffOptions | null | undefined,
  ): Diff;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oldIndex</span><span class="param-type">Index</span>
    <br>
    <p class="param-description">Index used for the &quot;oldFile&quot; side of the delta.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">newIndex</span><span class="param-type">Index</span>
    <br>
    <p class="param-description">Index used for the &quot;newFile&quot; side of the delta.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DiffOptions</span>
    <br>
    <p class="param-description">Describing options about how the diff should be executed.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">contextLines</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Set the number of unchanged lines that define the boundary of a hunk (and to display before and after).  The default value for this is 3.</p>
      </li>
      <li class="param-li">
        <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If pathspecs are specified, this flag means that they should be applied as an exact match instead of a fnmatch pattern.</p>
      </li>
      <li class="param-li">
        <span class="param-name">enableFastUntrackedDirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">When diff finds an untracked directory, to match the behavior of core Git, it scans the contents for ignored and untracked files. If all contents are ignored, then the directory is ignored; if any contents are not ignored, then the directory is untracked. This is extra work that may not matter in many cases.  This flag turns off that scan and immediately labels an untracked directory as untracked (changing the behavior to not match core git).</p>
      </li>
      <li class="param-li">
        <span class="param-name">forceBinary</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Treat all files as binary, disabling text diffs</p>
      </li>
      <li class="param-li">
        <span class="param-name">forceText</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Treat all files as text, disabling binary attributes and detection.</p>
      </li>
      <li class="param-li">
        <span class="param-name">idAbbrev</span><span class="param-type">number</span>
        <br>
        <p class="param-description">The default value for this is <code>core.abbrev</code> or 7 if unset.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreBlankLines</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore blank lines</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreCase</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether case insensitive filenames should be used.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreFilemode</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether file mode changes are ignored.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreSubmodules</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether all submodules should be treated as unmodified.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore all whitespace</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore changes in the amount of whitespace</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore whitespace at the end of line</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether ignored files are included.</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeTypechange</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If enabled, then Typechange delta records are generated.</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeTypechangeTrees</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Event with <code>includeTypechange</code>, the tree returned generally shows a deleted blob. This flag correctly labels the tree transitions as a typechange record with the <code>newFile</code>&#39;s mode set to tree.  Note that the tree SHA will not be available.</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether unmodified files are in the diff.</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUnreadable</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Include unreadable files in the diff</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUnreadableAsUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Include unreadable files in the diff as untracked files</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether untracked files are in the diff</p>
      </li>
      <li class="param-li">
        <span class="param-name">indentHeuristic</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Use a heuristic that takes indentation and whitespace into account which generally can produce better diffs when dealing with ambiguous diff hunks.</p>
      </li>
      <li class="param-li">
        <span class="param-name">interhunkLines</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Set the maximum number of unchanged lines between hunk boundaries before the hunks will be merged into one.  The default value for this is 0.</p>
      </li>
      <li class="param-li">
        <span class="param-name">maxSize</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Maximum size (in bytes) above which a blob will be marked as binary automatically.  A negative value will disable this entirely.  The default value for this is 512MB.</p>
      </li>
      <li class="param-li">
        <span class="param-name">minimal</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Take extra time to find the minimal diff</p>
      </li>
      <li class="param-li">
        <span class="param-name">newPrefix</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The virtual &quot;directory&quot; to prefix new file names with in hunk headers.  The default value for this is &quot;b&quot;.</p>
      </li>
      <li class="param-li">
        <span class="param-name">oldPrefix</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The virtual &quot;directory&quot; to prefix old file names with in hunk headers.  The default value for this is &quot;a&quot;.</p>
      </li>
      <li class="param-li">
        <span class="param-name">pathspecs</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">Add to the array of paths/fnmatch patterns to constrain the diff.</p>
      </li>
      <li class="param-li">
        <span class="param-name">patience</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Use the &quot;patience diff&quot; algorithm</p>
      </li>
      <li class="param-li">
        <span class="param-name">recurseIgnoredDirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether ignored directories are traversed deeply or not.</p>
      </li>
      <li class="param-li">
        <span class="param-name">recurseUntrackedDirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether untracked directories are traversed deeply or not.</p>
      </li>
      <li class="param-li">
        <span class="param-name">reverse</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Flag indicating whether the sides of the diff will be reversed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">showBinary</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Include the necessary deflate/delta information so that <code>git-apply</code> can apply given diff information to binary files.</p>
      </li>
      <li class="param-li">
        <span class="param-name">showUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">When generating output, include the names of unmodified files if they are included in the <code>Diff</code>. Normally these are skipped in the formats that list files (e.g. name-only, name-status, raw). Even with this these will not be included in the patch format.</p>
      </li>
      <li class="param-li">
        <span class="param-name">showUntrackedContent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">When generating patch text, include the content of untracked files.  This automatically turns on <code>includeUntracked</code> but it does not turn on <code>recurseUntrackedDirs</code>. Add that flag if you want the content of every single untracked file.</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipBinaryCheck</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Disable updating the <code>binary</code> flag in delta records. This is useful when iterating over a diff if you don&#39;t need hunk and data callbacks and want to avoid having to load a file completely.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">When diff finds a file in the working directory with stat information different from the index, but the OID ends up being the same, write the correct stat information into the index. Note: without this flag, diff will always leave the index untouched.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Diff</span>
    <br>
    <p class="param-description">Diff between two index objects.</p>
  </li>
</ul>
