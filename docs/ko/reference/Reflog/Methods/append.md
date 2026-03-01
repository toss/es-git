# append

reflog에 새 항목을 추가해요.

## 시그니처

```ts
class Reflog {
  append(newOid: string, committer: Signature, msg?: string | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newOid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">이 reflog 항목의 새 개체 ID(SHA1)</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">committer</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Signature</span>
    <br>
    <p class="param-description">이 reflog 항목의 커미터 서명</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 포함된 이메일</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 포함된 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">에포크로부터의 초 단위 시간</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">msg</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">이 reflog 항목의 선택적 메시지</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">OID가 유효하지 않거나 추가에 실패하면 오류를 던져요.</p>
  </li>
</ul>