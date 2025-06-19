# getMergeBases

두 커밋 사이의 모든 머지 베이스를 찾아요

## 시그니처

```ts
class Repository {
  getMergeBases(one: string, two: string): string[];
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">one</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">커밋 OID 중 하나</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">two</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">다른 커밋 OID</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">결과 OID를 저장할 배열</p>
  </li>
</ul>