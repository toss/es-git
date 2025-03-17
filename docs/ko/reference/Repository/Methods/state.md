# state

현재 리포지토리의 상태를 반환해요.

## 시그니처

```ts
class Repository {
  state(): RepositoryState;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">RepositoryState</span>
    <br>
    <p class="param-description">현재 리포지토리의 상태를 반환해요.</p>
    <p class="param-description">사용 가능한 상태는 다음과 같아요:
      <code>Clean</code>, <code>Merge</code>, <code>Revert</code>, <code>RevertSequence</code>, <code>CherryPick</code>,<br>
      <code>CherryPickSequence</code>, <code>Bisect</code>, <code>Rebase</code>, <code>RebaseInteractive</code>, <code>RebaseMerge</code>,<br>
      <code>ApplyMailbox</code>, <code>ApplyMailboxOrRebase</code>.</p>
  </li>
</ul>