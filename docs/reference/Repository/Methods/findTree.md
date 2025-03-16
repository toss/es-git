# findTree

Lookup a reference to one of the objects in a repository.

## Signature

```ts
class Repository {
  findTree(oid: string): Tree | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">ID(SHA1) to lookup.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Tree</span>
    <br>
    <p class="param-description">If it does not exist, returns  <code>null</code> .</p>
  </li>
</ul>