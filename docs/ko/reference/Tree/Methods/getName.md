# getName

Lookup a tree entry by its filename.

## 시그니처

```ts
class Tree {
  getName(filename: string): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">filename</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Filename of tree entry.</p>
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