# message

노트 메시지를 문자열로 가져와요.

## 시그니처

```ts
class Note {
  message(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">문자열인 노트 메시지</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">메시지가 utf-8이 아니면 오류를 던져요.</p>
  </li>
</ul>