# peel

Recursively peel an object until an object of the specified type is met.

## Signature

```ts
class GitObject {
  peel(objType: ObjectType): GitObject;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">objType</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">ObjectType</span>
    <br>
    <p class="param-description">If you pass <code>Any</code> as the target type, then the object will be peeled until the type changes (e.g. a tag will be chased until the referenced object is no longer a tag).</p>
    <p class="param-description">- <code>Any</code> : Any kind of git object<br>- <code>Commit</code> : An object which corresponds to a git commit<br>- <code>Tree</code> : An object which corresponds to a git tree<br>- <code>Blob</code> : An object which corresponds to a git blob<br>- <code>Tag</code> : An object which corresponds to a git tag</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object which recursively peeled.</p>
  </li>
</ul>