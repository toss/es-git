# type

Get the type of the object pointed by the entry.

## 시그니처

```ts
class TreeEntry {
  type(): ObjectType | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ObjectType</span>
    <br>
    <p class="param-description">The type of the object pointed by the entry.</p>
    <p class="param-description">- <code>Any</code> : Any kind of git object<br>- <code>Commit</code> : An object which corresponds to a git commit<br>- <code>Tree</code> : An object which corresponds to a git tree<br>- <code>Blob</code> : An object which corresponds to a git blob<br>- <code>Tag</code> : An object which corresponds to a git tag</p>
  </li>
</ul>