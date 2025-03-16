# type

Get the object type of object.

## Signature

```ts
class GitObject {
  type(): ObjectType | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ObjectType</span>
    <br>
    <p class="param-description">If the type is unknown, then  <code>null</code>  is returned.</p>
    <p class="param-description">- <code>Any</code> : Any kind of git object<br>- <code>Commit</code> : An object which corresponds to a git commit<br>- <code>Tree</code> : An object which corresponds to a git tree<br>- <code>Blob</code> : An object which corresponds to a git blob<br>- <code>Tag</code> : An object which corresponds to a git tag</p>
  </li>
</ul>