# openRepositoryFromWorktree

Open a repository from a worktree.

This will open the repository associated with the given worktree.

## Signature

```ts
function openRepositoryFromWorktree(worktree: Worktree): Repository;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">worktree</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Worktree</span>
    <br>
    <p class="param-description">Worktree to open repository from.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Repository</span>
    <br>
    <p class="param-description">Repository instance.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if opening the repository fails.</p>
  </li>
</ul>

## Examples

Open a repository from a worktree.

```ts
import { openWorktreeFromRepository, openRepositoryFromWorktree } from 'es-git';

const worktree = await openWorktreeFromRepository(repo);
const repo = openRepositoryFromWorktree(worktree);
```