# setVersion

Set index on-disk version.

## Signature

```ts
class Index {
  setVersion(version: number): number;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">version</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Version to set. Valid values are 2, 3, or 4. If 2 is given, git_index_write may write an index with version 3 instead, if necessary to accurately represent the index.</p>
  </li>
</ul>