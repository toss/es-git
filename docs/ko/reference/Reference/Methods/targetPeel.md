# targetPeel

이 레퍼런스가 가리키는 OID(Object ID)를 펼쳐서 반환해요.

펼친 OID는 직접 레퍼런스 중에서도 특정 개체(예: 태그된 커밋)를 가리킬 때만 적용돼요.

## 시그니처

```ts
class Reference {
  targetPeel(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">이 레퍼런스의 펼쳐진 OID를 반환해요.</p>
  </li>
</ul>