# reflogDelete

reflog을 삭제해요.

## 시그니처

```ts
class Repository {
  reflogDelete(name: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">삭제할 reflog이 있는 참조의 이름</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">삭제에 실패하면 오류를 던져요.</p>
  </li>
</ul>