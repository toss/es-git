# getMergeBaseOctopus

옥토퍼스 병합을 준비하기 위해 병합 베이스를 찾아요.

## 시그니처

```ts
class Repository {
  getMergeBaseOctopus(oids: string[]): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oids</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">커밋들의 OID</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">모든 커밋을 고려한 병합 베이스의 OID</p>
  </li>
</ul>