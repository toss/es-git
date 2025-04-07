# removePath

Remove an index entry corresponding to a file on disk.

If this file currently is the result of a merge conflict, this file will
no longer be marked as conflicting. The data about the conflict will be
moved to the "resolve undo" (REUC) section.

## Signature

```ts
class Index {
  removePath(path: string, options?: IndexRemoveOptions | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Relative file path to the repository&#39;s working directory.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | IndexRemoveOptions</span>
    <br>
    <p class="param-description">Options for remove an index entry.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">stage</span><span class="param-type">IndexStage</span>
        <br>
        <p class="param-description">- <code>Any</code> : Match any index stage.<br>- <code>Normal</code> : A normal staged file in the index.<br>- <code>Ancestor</code> : The ancestor side of a conflict.<br>- <code>Ours</code> : The &quot;ours&quot; side of a conflict.<br>- <code>Theirs</code> : The &quot;theirs&quot; side of a conflict.</p>
      </li>
    </ul>
  </li>
</ul>
