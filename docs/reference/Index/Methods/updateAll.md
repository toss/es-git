# updateAll

Update all index entries to match the working directory.

This scans the existing index entries and synchronizes them with the
working directory, deleting them if the corresponding working directory
file no longer exists otherwise updating the information (including
adding the latest version of file to the ODB if needed).

## Signature

```ts
class Index {
  updateAll(pathspecs: string[], options?: IndexUpdateAllOptions): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">pathspecs</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">A List of file names of shell glob patterns that will matched against files in the repository&#39;s working directory</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | IndexUpdateAllOptions</span>
    <br>
    <p class="param-description">Options for update all matching index entry.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">onMatch</span><span class="param-type">(args: IndexOnMatchCallbackArgs) =&gt; number</span>
        <br>
        <p class="param-description">If you provide a callback function, it will be invoked on each matching item in the index immediately before it is updated (either refreshed or removed depending on working directory state). Return 0 to proceed with updating the item, &gt; 0 to skip the item, and &lt; 0 to abort the scan.</p>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">This method will fail in bare index instances.</p>
  </li>
</ul>