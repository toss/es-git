# stashList

Get the list of stash states in the repository.

Returns a StashList object that provides access to all stashes in the repository.
The list is ordered with the most recent stash at index 0.

## Signature

```ts
class Repository {
  stashList(): StashList;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashList</span>
    <br>
    <p class="param-description">A container providing access to all stash entries in the repository.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

if (!stashList.isEmpty()) {
  console.log(`Found ${stashList.len()} stashes`);
  for (const stash of stashList.iter()) {
    console.log(`${stash.index()}: ${stash.message()}`);
  }
}
```