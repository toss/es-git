# worktree

리포지토리에 새 worktree를 추가해요.

## 시그니처

```ts
class Repository {
  worktree(name: string, path: string, options?: WorktreeAddOptions | null | undefined): Worktree;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">추가할 worktree의 이름.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">worktree가 생성될 경로.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">WorktreeAddOptions | null</span>
    <br>
    <p class="param-description">worktree 추가를 위한 옵션.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutExisting</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">활성화하면 worktree 이름과 일치하는 기존 브랜치를 체크아웃해요. 기본값은 <code>false</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">lock</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">활성화하면 새로 추가된 worktree가 잠금 상태가 돼요. 기본값은 <code>false</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">refName</span><span class="param-type">string</span>
        <br>
        <p class="param-description">새 worktree HEAD에 사용할 참조 이름. 기본값은 <code>null</code>이에요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Worktree</span>
    <br>
    <p class="param-description">새 worktree 인스턴스.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">worktree 추가에 실패하면 오류를 발생시켜요 (예: 경로가 이미 존재하거나, 잘못된 참조 이름이거나, 파일 시스템 오류가 있는 경우).</p>
  </li>
</ul>