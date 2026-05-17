# stashPop

Apply a single stashed state from the stash list and remove it from the list if successful.

This method combines `stashApply` and `stashDrop` into a single operation. It applies
the stash to your working directory and, if successful, removes it from the stash list.
If the application fails (e.g., due to conflicts), the stash remains in the list.

## Signature

```ts
class Repository {
  stashPop(index: number, options?: StashApplyOptions): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">The index of the stash to pop (0 is the most recent).</p>
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

// Pop the most recent stash
repo.stashPop(0);

// Pop with options
repo.stashPop(0, { reinstantiateIndex: true });
```