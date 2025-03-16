# targetType

Get the ObjectType of the tagged object of a tag.

## 시그니처

```ts
class Tag {
  targetType(): ObjectType | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ObjectType</span>
    <br>
    <p class="param-description">ObjectType of the tagged object of a tag.</p>
    <p class="param-description">- <code>Any</code> : Any kind of git object<br>- <code>Commit</code> : An object which corresponds to a git commit<br>- <code>Tree</code> : An object which corresponds to a git tree<br>- <code>Blob</code> : An object which corresponds to a git blob<br>- <code>Tag</code> : An object which corresponds to a git tag</p>
  </li>
</ul>