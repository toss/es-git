# resolveSignature

mailmap을 사용하여 서명을 정규 형식으로 확인해요.

정규 이름과 이메일이 포함된 새 서명을 반환해요.

## 시그니처

```ts
class Mailmap {
  resolveSignature(signature: SignaturePayload): Signature;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">SignaturePayload</span>
    <br>
    <p class="param-description">확인할 서명</p>
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
        <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">offset</span><span class="param-type">number</span>
            <br>
            <p class="param-description">시간대 오프셋(분 단위)</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">epoch부터 초 단위 시간</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description">정규 이름과 이메일이 포함된 확인된 서명</p>
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
        <p class="param-description">epoch부터 초 단위 시간</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">작업이 실패한 경우 오류.</p>
  </li>
</ul>