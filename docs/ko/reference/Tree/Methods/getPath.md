# getPath

Retrieve a tree entry contained in a tree or in any of its subtrees,
given its relative path.

## 시그니처

```ts
class Tree {
  getPath(path: string): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">Tree entry.</p>
  </li>
</ul>