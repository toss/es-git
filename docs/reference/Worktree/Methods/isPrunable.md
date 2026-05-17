# isPrunable

Check if the worktree is prunable.

## Signature

```ts
class Worktree {
  isPrunable(options?: WorktreePruneOptions | null | undefined): boolean;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">worktreePruneOptions</span><span class="param-type">WorktreePruneOptions | null</span>
    <br>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">locked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Controls whether locked worktrees will be pruned.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">valid</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Controls whether valid (still existing on the filesystem) worktrees will be pruned.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">workingTree</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Controls whether the actual working tree on the filesystem is recursively removed.  Defaults to <code>false</code>.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description"><code>true</code>  if the worktree is prunable,  <code>false</code>  otherwise.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if checking fails.</p>
  </li>
</ul>