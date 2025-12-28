# submodule

Set up a new git submodule for checkout.

This does "git submodule add" up to the fetch and checkout of the
submodule contents. It preps a new submodule, creates an entry in
`.gitmodules` and creates an empty initialized repository either at the
given path in the working directory or in `.git/modules` with a gitlink
from the working directory to the new repo.

To fully emulate "git submodule add" call this function, then `open()`
the submodule repo and perform the clone step as needed. Lastly, call
`addFinalize()` to wrap up adding the new submodule and `.gitmodules`
to the index to be ready to commit.

## Signature

```ts
class Repository {
  submodule(
    url: string,
    path: string,
    useGitlink?: boolean | null | undefined,
  ): Submodule;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">URL for the submodule&#39;s remote.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Path at which the submodule should be created.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">useGitlink</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">Should workdir contain a gitlink to the repo in <code>.git/modules</code> vs. repo directly in workdir.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Submodule</span>
    <br>
    <p class="param-description">The submodule.</p>
  </li>
</ul>