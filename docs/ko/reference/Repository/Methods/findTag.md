# findTag

리포지토리에서 특정 해시(prefix hash)로 태그를 조회해요.

## 시그니처

```ts
class Repository {
  findTag(oid: string): Tag | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 태그의 해시(prefix hash)예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Tag</span>
    <br>
    <p class="param-description">주어진 해시에 해당하는 태그를 반환해요. 태그가 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>