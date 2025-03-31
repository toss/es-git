# discoverRepository

Attempt to open an already-existing repository at or above `path`.

This starts at `path` and looks up the filesystem hierarchy
until it finds a repository.

## Signature

```ts
function discoverRepository(path: string, signal?: AbortSignal | null | undefined): Promise<Repository>;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Directory path to discover repository.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">Git repository.</p>
  </li>
</ul>
