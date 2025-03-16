# toObject

Convert a tree entry to the object it points to.

## 시그니처

```ts
class TreeEntry {
  toObject(repo: Repository): GitObject;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">repo</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Repository</span>
    <br>
    <p class="param-description">Repository which this tree entry belongs to.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object that pointed by the entry.</p>
  </li>
</ul>