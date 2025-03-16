# revparseSingle

주어진 리비전 문자열을 기반으로 단일 개체의 OID를 찾아요.

## 시그니처

```ts
class Repository {
  revparseSingle(spec: string): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">spec</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 리비전 문자열이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">조회된 개체의 OID(Object ID)를 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">개체가 존재하지 않으면 오류를 발생시켜요.</p>
  </li>
</ul>