# commitSigned

외부에서 서명한 커밋 내용을 저장해요.

서명된 커밋을 객체 데이터베이스에 저장하지만 `HEAD`나 다른 레퍼런스는 갱신하지 않아요.
`signatureField`를 생략하면 Git의 기본 필드인 `gpgsig`를 사용해요.

## 시그니처

```ts
class Repository {
  commitSigned(commitContent: string, signature: string, signatureField?: string | null | undefined): string;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitContent</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>commitCreateBuffer()</code>가 반환한 커밋 내용이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">커밋 내용에 대한 외부 서명이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signatureField</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">서명 필드 이름이에요. 기본값은 <code>gpgsig</code>예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">생성한 커밋의 ID(SHA1)예요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">커밋 내용, 서명 또는 서명 필드가 유효하지 않을 때 발생해요.</p>
  </li>
</ul>
