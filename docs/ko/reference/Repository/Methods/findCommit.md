# findCommit

리포지토리에서 특정 커밋을 조회해요.

## 시그니처

```ts
class Repository {
  findCommit(oid: string): Commit | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 커밋의 ID(SHA-1)예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Commit</span>
    <br>
    <p class="param-description">주어진 OID에 해당하는 커밋을 반환해요. 커밋이 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>