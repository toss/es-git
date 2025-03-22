# findSimilar

Transform a diff marking file renames, copies, etc.

This modifies a diff in place, replacing old entries that look like
renames or copies with new entries reflecting those changes. This also
will, if requested, break modified files into add/remove pairs if the
amount of change is above a threshold.

## Signature

```ts
class Diff {
  findSimilar(options?: DiffFindOptions): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DiffFindOptions</span>
    <br>
    <p class="param-description">Options for finding diff.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">all</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Turn on all finding features.</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Actually split large rewrites into delete/add pairs</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewritesForRenamesOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Do not break rewrites unless they contribute to a rename.  Normally, <code>breakRewrites</code> and <code>rewrites</code> will measure the self-similarity of modified files and split the ones that have changed a lot into a delete/add pair. Then the sides of that pair will be considered candidates for rename and copy detection  If you add this flag in and the split pair is not used for an actual rename or copy, then the modified record will be restored to a regular modified record instead of being split.</p>
      </li>
      <li class="param-li">
        <span class="param-name">breakRewriteThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Similarity to split modify into delete/add pair (default 60)</p>
      </li>
      <li class="param-li">
        <span class="param-name">copies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Look for copies?</p>
      </li>
      <li class="param-li">
        <span class="param-name">copiesFromUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Consider unmodified as copy sources?  For this to work correctly, use <code>includeUnmodified</code> when the initial diff is being generated.</p>
      </li>
      <li class="param-li">
        <span class="param-name">copyThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Similarity to consider a file copy (default 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">dontIgnoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Measure similarity including all data</p>
      </li>
      <li class="param-li">
        <span class="param-name">exactMatchOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Measure similarity only by comparing SHAs (fast and cheap)</p>
      </li>
      <li class="param-li">
        <span class="param-name">forUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Find renames/copies for untracked items in working directory.  For this to work correctly use the <code>includeUntracked</code> option when the initial diff is being generated.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreLeadingWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Measure similarity ignoring leading whitespace (default)</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Measure similarity ignoring all whitespace</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeUnmodified</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Remove any unmodified deltas after find_similar is done.  Using <code>copiesFromUnmodified</code> to emulate the <code>--find-copies-harder</code> behavior requires building a diff with the <code>includeUnmodified</code> flag. If you do not want unmodified records in the final result, pas this flag to have them removed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameFromRewriteThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Similarity of modified to be eligible rename source (default 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Maximum similarity sources to examine for a file (somewhat like git-diff&#39;s <code>-l</code> option or <code>diff.renameLimit</code> config)  Defaults to 200</p>
      </li>
      <li class="param-li">
        <span class="param-name">renames</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Look for renames?</p>
      </li>
      <li class="param-li">
        <span class="param-name">renamesFromRewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Consider old side of modified for renames?</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Similarity to consider a file renamed (default 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">rewrites</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Mark significant rewrites for split.</p>
      </li>
    </ul>
  </li>
</ul>