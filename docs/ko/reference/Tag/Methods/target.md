# target

Get the tagged object of a tag.

This method performs a repository lookup for the given object and
returns it.

## 시그니처

```ts
class Tag {
  target(): GitObject;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Tagged git object of a tag.</p>
  </li>
</ul>