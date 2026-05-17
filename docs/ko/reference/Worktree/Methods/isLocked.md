# isLocked

워크트리가 잠겨 있는지 확인해요.

## 시그니처

```ts
class Worktree {
  isLocked(): WorktreeLockStatus;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">WorktreeLockStatus</span>
    <br>
    <p class="param-description">워크트리의 잠금 상태.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">reason</span><span class="param-type">string</span>
        <br>
        <p class="param-description">선택적 메시지와 함께 잠긴 워크트리.</p>
      </li>
      <li class="param-li">
        <span class="param-name">status</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">WorktreeLockStatusType</span>
        <br>
        <p class="param-description">잠금 해제된 워크트리.</p>
        <p class="param-description">워크트리의 잠금 상태.</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">잠금 상태 확인에 실패하면 오류를 발생시켜요.</p>
  </li>
</ul>