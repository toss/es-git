# iter

Returns an iterator over the stash entries in this list.

The iterator yields stash entries in order from newest (index 0) to oldest.

## Signature

```ts
class StashList {
  iter(): StashListIter;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashListIter</span>
    <br>
    <p class="param-description">An iterator that yields StashEntry objects.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

// Iterate over stashes
for (const stash of stashList.iter()) {
  console.log(`${stash.index()}: ${stash.message()}`);
}
```