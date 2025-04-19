# findBranch

Lookup a branch by its name in a repository.

## Signature

```ts
class Repository {
  findBranch(name: string, branchType: BranchType): Branch | null;
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
    <span class="param-type">null | Branch</span>
    <br>
    <p class="param-description">A found branch.</p>
  </li>
</ul>