# getCommit

Lookup a reference to one of the commits in a repository.

## Signature

```ts
class Repository {
  getCommit(oid: string): Commit;
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
    <span class="param-type">Commit</span>
    <br>
    <p class="param-description">Commit instance found by oid.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the commit does not exist.</p>
  </li>
</ul>