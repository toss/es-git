# openWorktreeFromRepository

Open a worktree from a repository.

This will open the worktree associated with the given repository if the
repository is a worktree.

## Signature

```ts
function openWorktreeFromRepository(repo: Repository): Worktree;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">repo</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Repository</span>
    <br>
    <p class="param-description">Repository to open worktree from.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Worktree</span>
    <br>
    <p class="param-description">Worktree instance.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the repository is not a worktree or if opening fails.</p>
  </li>
</ul>

## Examples

Open a worktree from a repository.

```ts
import { openRepository, openWorktreeFromRepository } from 'es-git';

const repo = await openRepository('.');
const worktree = openWorktreeFromRepository(repo);
```