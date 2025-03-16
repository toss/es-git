# toObject

트리 항목(tree entry)을 해당 항목이 가리키는 GitObject로 변환해요.

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
    <p class="param-description">이 트리 항목(tree entry)이 속한 리포지토리예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">이 항목이 가리키고 있는 Git 개체예요.</p>
  </li>
</ul>
