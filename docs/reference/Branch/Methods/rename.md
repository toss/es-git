# rename

Move/rename an existing local branch reference.

## Signature

```ts
class Branch {
  rename(newBranchName: string, options?: BranchRenameOptions | null | undefined): Branch;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newBranchName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Branch name to move/rename.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | BranchRenameOptions</span>
    <br>
    <p class="param-description">Options for move/rename branch.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If the force flag is not enabled, and there&#39;s already a branch with the given name, the renaming will fail.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">Move/renamed branch.</p>
  </li>
</ul>