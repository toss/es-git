# hideRef

주어진 레퍼런스가 가리키는 OID를 숨겨요.

이 메서드를 사용하면 해당 레퍼런스가 가리키는 커밋과 그 조상 커밋들이 Revwalk 결과에서 제외돼요.

## 시그니처

```ts
class Revwalk {
  hideRef(reference: string): this;
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