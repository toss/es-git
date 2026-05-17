# isLocked

Check if the worktree is locked.

## Signature

```ts
class Worktree {
  isLocked(): WorktreeLockStatus;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">WorktreeLockStatus</span>
    <br>
    <p class="param-description">Lock status of the worktree.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">reason</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Worktree is locked with the optional message</p>
      </li>
      <li class="param-li">
        <span class="param-name">status</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">WorktreeLockStatusType</span>
        <br>
        <p class="param-description">Worktree is Unlocked</p>
        <p class="param-description">Lock Status of a worktree</p>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if checking the lock status fails.</p>
  </li>
</ul>