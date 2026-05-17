# stashDrop

Remove a single stashed state from the stash list.

This permanently deletes a stash entry. The stash is removed from the list and
cannot be recovered. All subsequent stashes will be reindexed (e.g., stash@{2}
becomes stash@{1} after dropping stash@{1}).

## Signature

```ts
class Repository {
  stashDrop(index: number): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">The index of the stash to drop (0 is the most recent).</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the stash index is invalid.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Drop the most recent stash
repo.stashDrop(0);

// Drop the third stash
repo.stashDrop(2);
```