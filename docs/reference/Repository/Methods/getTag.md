# getTag

Lookup a tag object by prefix hash from the repository.

## Signature

```ts
class Repository {
  getTag(oid: string): Tag;
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

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if tag does not exist.</p>
  </li>
</ul>