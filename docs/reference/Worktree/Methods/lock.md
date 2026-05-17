# lock

Lock the worktree.

## Signature

```ts
class Worktree {
  lock(reason?: string | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reason</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">Optional reason for locking the worktree.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if locking fails.</p>
  </li>
</ul>