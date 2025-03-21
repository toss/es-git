# openRepository

Attempt to open an already-existing repository at `path`.

## Signature

```ts
function openRepository(
  path: string,
  options?: RepositoryOpenOptions,
  signal?: AbortSignal,
): Promise<Repository>;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Directory path to repository already-existing.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryOpenOptions</span>
    <br>
    <p class="param-description">Options which can be used to configure how a repository is initialized.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">bare</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If this option is <code>true</code>, force opening the repository as bare event if it isn&#39;t, ignoring any working directory, and defer loading the repository configuration for performance.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ceilingDirs</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">ceiling_dirs specifies a list of paths that the search through parent directories will stop before entering.</p>
      </li>
      <li class="param-li">
        <span class="param-name">crossFs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If this option is <code>true</code>, the search through parent directories will not cross a filesystem boundary (detected when the stat st_dev field changes).</p>
      </li>
      <li class="param-li">
        <span class="param-name">fromEnv</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If this option is <code>true</code>, <code>open</code> will ignore other options and <code>ceilingDirs</code>, and respect the same environment variables git does. Note, however, that <code>path</code> overrides <code>$GIT_DIR</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noDotgit</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If this options is <code>true</code>, don&#39;t try appending <code>/.git</code> to <code>path</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noSearch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If this option is <code>true</code>, the path must point directly to a repository; otherwise, this may point to a subdirectory of a repository, and <code>open</code> will search up through parent directories.</p>
      </li>
    </ul>
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
    <p class="param-description">Opened repository.</p>
  </li>
</ul>

## Examples

Basic example.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```

Open bare repository.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo.git', {
  bare: true,
});
```