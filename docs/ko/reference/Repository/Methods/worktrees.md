# worktrees

리포지토리의 모든 worktree를 나열해요.

## 시그니처

```ts
class Repository {
  worktrees(): string[];
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">worktree 이름의 배열.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">worktree 나열이 실패하면 오류를 발생시켜요 (예: 파일 시스템 오류 또는 리포지토리 손상).</p>
  </li>
</ul>