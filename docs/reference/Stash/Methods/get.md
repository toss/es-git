# get

Gets a stash entry from this list at the specified index.

## Signature

```ts
class StashList {
  get(index: number): StashEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Index of the stash entry to get (0-based, where 0 is the most recent).</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashEntry | null</span>
    <br>
    <p class="param-description">A stash entry from this list at the specified index, or  <code>null</code>  if the index is out of bounds.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

// Get the most recent stash
const stash = stashList.get(0);
if (stash) {
  console.log(stash.message());
}
```