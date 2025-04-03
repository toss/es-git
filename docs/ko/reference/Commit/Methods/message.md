# message

커밋의 전체 메시지를 가져와요.

반환되는 메시지는 잠재적인 선행 개행을 제거하여 약간 다듬어진 형태로 제공돼요.

## 시그니처

```ts
class Commit {
  message(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">커밋의 전체 메시지를 반환해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">올바른 UTF-8 인코딩이 아닐 경우 에러를 발생시켜요.</p>
  </li>
</ul>