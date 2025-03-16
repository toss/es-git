# get

Lookup a tree entry by its position in the tree.

## 시그니처

```ts
class Tree {
  get(index: number): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
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