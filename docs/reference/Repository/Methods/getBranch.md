# getBranch

Lookup a branch by its name in a repository.

## Signature

```ts
class Repository {
  getBranch(name: string, branchType: BranchType): Branch;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">A branch name.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">branchType</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">BranchType</span>
    <br>
    <p class="param-description">Branch type to lookup.</p>
    <p class="param-description">- <code>Local</code> : A local branch not on a remote.<br>- <code>Remote</code> : A branch for a remote.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">A found branch.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if branch does not exist.</p>
  </li>
</ul>