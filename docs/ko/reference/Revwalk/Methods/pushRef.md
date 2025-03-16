# pushRef

주어진 레퍼런스가 가리키는 OID를 추가해요.

## 시그니처

```ts
class Revwalk {
  pushRef(reference: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reference</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">커밋을 가리키는 레퍼런스 이름이에요.</p>
  </li>
</ul>