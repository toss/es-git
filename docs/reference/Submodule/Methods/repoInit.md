# repoInit

Set up the subrepository for a submodule in preparation for clone.

This function can be called to init and set up a submodule repository
from a submodule in preparation to clone it from its remote.

## Signature

```ts
class Submodule {
  repoInit(
    useGitlink?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<Repository>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">useGitlink</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">Should the workdir contain a gitlink to the repo in <code>.git/modules</code> vs. repo directly in workdir.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Optional AbortSignal to cancel the operation.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">The repository.</p>
  </li>
</ul>