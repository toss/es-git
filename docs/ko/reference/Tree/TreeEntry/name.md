# name

Get the filename of a tree entry.

## 시그니처

```ts
class TreeEntry {
  name(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The filename of a tree entry.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the name is not valid utf-8.</p>
  </li>
</ul>