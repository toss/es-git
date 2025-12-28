# addToIndex

Add current submodule HEAD commit to index of superproject.

## Signature

```ts
class Submodule {
  addToIndex(writeIndex?: boolean | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">writeIndex</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">If is true, then the index file will be immediately written. Otherwise, you must explicitly call <code>write()</code> on an <code>Index</code> later on.</p>
  </li>
</ul>