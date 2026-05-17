# committerWithMailmap

mailmap을 사용하여 정규 이름과 이메일에 매핑하고, 이 커밋의 커미터를 가져와요.

## 시그니처

```ts
class Commit {
  committerWithMailmap(mailmap: Mailmap): Signature;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">mailmap</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Mailmap</span>
    <br>
    <p class="param-description">매핑에 사용할 mailmap</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description">매핑이 적용된 이 커밋의 커미터 서명</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명의 이메일.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명의 이름.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">에포크 기준, 초 단위 시간</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">작업이 실패하면 오류가 발생해요.</p>
  </li>
</ul>