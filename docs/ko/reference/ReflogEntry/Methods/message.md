# message

이 reflog 항목의 메시지를 가져와요.

## 시그니처

```ts
class ReflogEntry {
  message(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">이 reflog 항목의 메시지. 메시지가 없으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">메시지가 유효한 utf-8이 아니면 오류를 throw해요.</p>
  </li>
</ul>