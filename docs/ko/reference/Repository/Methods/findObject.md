# findObject

리포지토리에서 특정 Git 개체를 조회해요.

## 시그니처

```ts
class Repository {
  findObject(oid: string): GitObject | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Git object ID(SHA1) to lookup.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | GitObject</span>
    <br>
    <p class="param-description">Git object. Returns  <code>null</code>  if the object does not exist.</p>
  </li>
</ul>