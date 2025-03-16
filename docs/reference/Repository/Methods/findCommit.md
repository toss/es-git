# findCommit

Lookup a reference to one of the commits in a repository.

Returns `null` if the commit does not exist.

## Signature

```ts
class Repository {
  findCommit(oid: string): Commit | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Commit ID(SHA1) to lookup.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Commit</span>
    <br>
    <p class="param-description">Commit instance found by oid. Returns  <code>null</code>  if the commit does not exist.</p>
  </li>
</ul>