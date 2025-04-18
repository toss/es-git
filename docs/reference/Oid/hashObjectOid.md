# hashObjectOid

Hashes the provided data as an object of the provided type, and returns
an Oid corresponding to the result. This does not store the object
inside any object database or repository.

## Signature

```ts
function hashObjectOid(objType: ObjectType, bytes: Buffer): string;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">objType</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">ObjectType</span>
    <br>
    <p class="param-description">Git object type.</p>
    <p class="param-description">- <code>Any</code> : Any kind of git object<br>- <code>Commit</code> : An object which corresponds to a git commit<br>- <code>Tree</code> : An object which corresponds to a git tree<br>- <code>Blob</code> : An object which corresponds to a git blob<br>- <code>Tag</code> : An object which corresponds to a git tag</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">bytes</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
    <br>
    <p class="param-description">Data to hashed.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Hashed string.</p>
  </li>
</ul>