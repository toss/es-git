# worktree

Add a new worktree to the repository.

## Signature

```ts
class Repository {
  worktree(name: string, path: string, options?: WorktreeAddOptions | null | undefined): Worktree;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name of the worktree to add.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Path where the worktree should be created.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">WorktreeAddOptions | null</span>
    <br>
    <p class="param-description">Options for adding the worktree.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutExisting</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If enabled, this will checkout the existing branch matching the worktree name.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">lock</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If enabled, this will cause the newly added worktree to be locked.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">refName</span><span class="param-type">string</span>
        <br>
        <p class="param-description">reference name to use for the new worktree HEAD  Defaults to <code>null</code>.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Worktree</span>
    <br>
    <p class="param-description">New worktree instance.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if adding the worktree fails (e.g., path already exists, invalid reference name, or filesystem errors).</p>
  </li>
</ul>