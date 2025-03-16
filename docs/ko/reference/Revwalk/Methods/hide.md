# hide

이 메서드를 사용하면 지정한 커밋을 기준으로 그 이전의 커밋들이 탐색 대상에서 제외돼요.

## 시그니처

```ts
class Revwalk {
  hide(oid: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Revwalk에서 제외할 커밋의 OID예요.</p>
  </li>
</ul>