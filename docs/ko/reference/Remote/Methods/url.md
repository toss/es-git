# url

리모트의 URL을 가져와요.

## 시그니처

```ts
class Remote {
  url(): string;
}
```

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">URL이 유효한 UTF-8 문자열이 아니면 오류를 발생시켜요.</p>
  </li>
</ul>