# reflogRename

Rename a reflog.

## Signature

```ts
class Repository {
  reflogRename(oldName: string, newName: string): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oldName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Old name of the reference.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">newName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">New name of the reference.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if renaming fails.</p>
  </li>
</ul>