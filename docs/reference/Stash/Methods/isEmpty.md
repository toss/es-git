# isEmpty

Check if the stash list is empty.

## Signature

```ts
class StashList {
  isEmpty(): boolean;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  if there are no stash entries in this repository.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

if (stashList.isEmpty()) {
  console.log('No stashes found');
} else {
  console.log(`Found ${stashList.len()} stashes`);
}
```