# lock

워크트리를 잠가요.

## 시그니처

```ts
class Worktree {
  lock(reason?: string | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reason</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">워크트리를 잠그는 선택적 이유.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">잠금에 실패하면 오류를 발생시켜요.</p>
  </li>
</ul>