# getMergeBasesMany

커밋 목록이 주어졌을 때 모든 병합 베이스를 찾아요

## 시그니처

```ts
class Repository {
  getMergeBasesMany(oids: string[]): string[];
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oids</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">커밋의 OID들</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">결과 OID들을 저장할 배열</p>
  </li>
</ul>