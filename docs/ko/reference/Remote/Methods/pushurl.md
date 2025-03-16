# pushurl

리모트의 푸시(push) URL을 가져와요.

## 시그니처

```ts
class Remote {
  pushurl(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">푸시 URL이 설정되지 않았으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">URL이 유효한 UTF-8 문자열이 아니면 오류를 발생시켜요.</p>
  </li>
</ul>