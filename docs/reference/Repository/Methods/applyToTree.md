# applyToTree

Apply a Diff to the provided tree, and return the resulting Index.

## Signature

```ts
class Repository {
  applyToTree(
    tree: Tree,
    diff: Diff,
    options?: ApplyOptions | null | undefined
  ): Index;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">The tree to apply the diff to</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">The diff to apply</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | ApplyOptions</span>
    <br>
    <p class="param-description">The options for the apply</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">check</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Don&#39;t actually make changes, just test that the patch applies.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">The postimage of the application</p>
  </li>
</ul>