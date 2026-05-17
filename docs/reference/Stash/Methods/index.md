# index

Get the index of this stash entry.

The index represents the position of this stash in the stash stack, where 0 is the most recent stash.

## Signature

```ts
class StashEntry {
  index(): number;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">Index of this stash entry (0-based, with 0 being the most recent).</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();
const stash = stashList.get(0);
console.log(stash?.index()); // 0
```