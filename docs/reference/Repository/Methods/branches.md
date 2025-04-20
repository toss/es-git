# branches

Create an iterator which loops over the requested branches.

## Signature

```ts
class Repository {
  branches(filter?: BranchesFilter | null | undefined): Branches;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">filter</span><span class="param-type">null | BranchesFilter</span>
    <br>
    <p class="param-description">Filter for the branches iterator.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">type</span><span class="param-type">BranchType</span>
        <br>
        <p class="param-description">Branch type to filter.</p>
        <p class="param-description">- <code>Local</code> : A local branch not on a remote.<br>- <code>Remote</code> : A branch for a remote.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branches</span>
    <br>
    <p class="param-description">An iterator which loops over the requested branches.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');

for (const branch of repo.branches()) {
  console.log(branch.type); // "Local"
  console.log(branch.name); // "main"
}
```