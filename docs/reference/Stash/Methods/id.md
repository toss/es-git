# id

Get the id (SHA1) of this stash entry.

Each stash is stored as a commit object, and this returns the commit SHA.

## Signature

```ts
class StashEntry {
  id(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The 40-character hexadecimal SHA1 hash of the stash commit.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();
const stash = stashList.get(0);
console.log(stash?.id()); // e.g., "a1b2c3d4e5f6..."
```