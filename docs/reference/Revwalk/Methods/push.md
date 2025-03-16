# push

Mark a commit to start traversal from.

The given OID must belong to a commitish on the walked repository.

The given commit will be used as one of the roots when starting the
revision walk. At least one commit must be pushed onto the walker before
a walk can be started.

## Signature

```ts
class Revwalk {
  push(oid: string): this;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">OID which belong to a commitish on the walked repository.</p>
  </li>
</ul>