# submoduleSetBranch

Set the branch for the submodule in the configuration

After calling this, you may wish to call `Submodule#sync()` to write
the changes to the checked out submodule repository.

## Signature

```ts
class Repository {
  submoduleSetBranch(name: string, branchName: string): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the submodule to configure.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">branchName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Branch that should be used for the submodule</p>
  </li>
</ul>