# committer

이 커밋의 커미터(committer)를 가져와요.

## 시그니처

```ts
class Commit {
  committer(): Signature;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description">커밋 커미터의 서명을 반환해요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 표시될 이메일 주소예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 표시될 이름이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">유닉스 에포크(1970년 1월 1일 00:00:00 UTC)부터 경과한 시간(초 단위)이에요.</p>
      </li>
    </ul>
  </li>
</ul>