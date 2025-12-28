# remove

Remove an entry from the reflog.

## Signature

```ts
class Reflog {
  remove(i: number, rewritePreviousEntry?: boolean): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">i</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Index of the entry to remove.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">rewritePreviousEntry</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">Whether to rewrite the previous entry. Defaults to <code>false</code>.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the index is invalid or if removal fails.</p>
  </li>
</ul>