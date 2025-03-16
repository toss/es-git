# findObject

Lookup a reference to one of the objects in a repository.

## Signature

```ts
class Repository {
  findObject(oid: string): GitObject | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Git object ID(SHA1) to lookup.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | GitObject</span>
    <br>
    <p class="param-description">Git object. Returns  <code>null</code>  if the object does not exist.</p>
  </li>
</ul>