# findTree

리포지토리에서 특정 트리 개체를 조회해요.

## 시그니처

```ts
class Repository {
  findTree(oid: string): Tree | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 트리 개체의 ID(SHA-1)예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Tree</span>
    <br>
    <p class="param-description">주어진 OID에 해당하는 트리 개체를 반환해요. 개체가 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>