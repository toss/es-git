# finish

모든 패치가 적용되면 현재 진행 중인 리베이스를 완료해요.

## 시그니처

```ts
class Rebase {
  finish(signature?: SignaturePayload | undefined | null): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-type">null | SignaturePayload</span>
    <br>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 있는 이메일이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">서명에 있는 이름이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">offset</span><span class="param-type">number</span>
            <br>
            <p class="param-description">시간대 오프셋을 분 단위로 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">에포크부터 초 단위 시간이에요.</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>