# removeAll

Remove all matching index entries.

## Signature

```ts
class Index {
  removeAll(pathspecs: string[], options?: IndexRemoveAllOptions | null | undefined): void;
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
    <span class="param-name">options</span><span class="param-type">null | IndexRemoveAllOptions</span>
    <br>
    <p class="param-description">Options for remove all matching index entry.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">onMatch</span><span class="param-type">(args: IndexOnMatchCallbackArgs) =&gt; number</span>
        <br>
        <p class="param-description">If you provide a callback function, it will be invoked on each matching item in the index immediately before it is removed. Return 0 to remove the item, &gt; 0 to skip the item, and &lt; 0 to abort the scan.</p>
      </li>
    </ul>
  </li>
</ul>
