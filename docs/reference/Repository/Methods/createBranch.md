# createBranch

Create a new branch pointing at a target commit

A new direct reference will be created pointing to this target commit.

## Signature

```ts
class Repository {
  createBranch(
    branchName: string,
    target: Commit,
    options?: CreateBranchOptions | null | undefined,
  ): Branch;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">branchName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name for the new branch.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">Target commit which will be pointed by this branch.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateBranchOptions</span>
    <br>
    <p class="param-description">Options for create branch.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If <code>force</code> is true and a reference already exists with the given name, it&#39;ll be replaced.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">Newly created branch.</p>
  </li>
</ul>