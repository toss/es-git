# worktrees

List all worktrees in the repository.

## Signature

```ts
class Repository {
  worktrees(): string[];
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">Array of worktree names.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if listing worktrees fails (e.g., filesystem errors or repository corruption).</p>
  </li>
</ul>