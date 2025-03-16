# findTag

Lookup a tag object by prefix hash from the repository.

## Signature

```ts
class Repository {
  findTag(oid: string): Tag | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Prefix hash.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Tag</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if tag does not exist.</p>
  </li>
</ul>