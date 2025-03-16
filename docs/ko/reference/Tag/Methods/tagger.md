# tagger

태그를 작성한 사람(저자)의 정보를 가져와요.

## 시그니처

```ts
class Tag {
  tagger(): Signature | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Signature</span>
    <br>
    <p class="param-description">저자가 지정되지 않은 경우 <code>null</code>을 반환해요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">저자의 이메일이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">저자의 이름이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">Epoch(1970-01-01 00:00:00 UTC) 이후 초 단위의 타임스탬프예요.</p>
      </li>
    </ul>
  </li>
</ul>
