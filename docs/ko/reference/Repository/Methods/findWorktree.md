# findWorktree

이름으로 worktree를 찾아요.

## 시그니처

```ts
class Repository {
  findWorktree(name: string): Worktree;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">찾을 worktree의 이름.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Worktree</span>
    <br>
    <p class="param-description">Worktree 인스턴스.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">worktree를 찾을 수 없거나 열기에 실패하면 오류를 던져요.</p>
  </li>
</ul>