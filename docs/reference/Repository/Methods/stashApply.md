# stashApply

Apply a single stashed state from the stash list.

This method applies the changes from a stash entry to your working directory.
Unlike `stashPop`, this does not remove the stash from the list after applying.
Conflicts may occur if the stashed changes conflict with the current working directory.

## Signature

```ts
class Repository {
  stashApply(index: number, options?: StashApplyOptions): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">The index of the stash to apply (0 is the most recent).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">StashApplyOptions | null</span>
    <br>
    <p class="param-description">Options for applying the stash.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">reinstantiateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Whether to reinstall the index from the stash. If true, the index state recorded in the stash is also restored. Default: false</p>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the stash index is invalid or if there are conflicts during application.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Apply the most recent stash
repo.stashApply(0);

// Apply with options
repo.stashApply(0, { reinstantiateIndex: true });
```