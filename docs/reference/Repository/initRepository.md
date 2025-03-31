# initRepository

Creates a new repository in the specified folder.

## Signature

```ts
function initRepository(
  path: string,
  options?: RepositoryInitOptions | null | undefined,
  signal?: AbortSignal | null | undefined,
): Promise<Repository>;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Directory path to create new repository.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryInitOptions</span>
    <br>
    <p class="param-description">Options which can be used to configure how a repository is initialized.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">bare</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Create a bare repository with no working directory.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">description</span><span class="param-type">string</span>
        <br>
        <p class="param-description">If set, this will be used to initialize the &quot;description&quot; file in the repository instead of using the template content.</p>
      </li>
      <li class="param-li">
        <span class="param-name">externalTemplate</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Enable or disable using external templates.  If enabled, then the <code>template_path</code> option will be queried first, then <code>init.templatedir</code> from the global config, and finally <code>/usr/share/git-core-templates</code> will be used (if it exists).  Defaults to <code>true</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">initialHead</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The name of the head to point HEAD at.  If not configured, this will be taken from your git configuration. If this begins with <code>refs/</code> it will be used verbatim; otherwise <code>refs/heads/</code> will be prefixed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mkdir</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Make the repo path (and workdir path) as needed. The &quot;.git&quot; directory will always be created regardless of this flag.  Defaults to <code>true</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mkpath</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Make the repo path (and workdir path) as needed. The &quot;.git&quot; directory will always be created regardless of this flag.  Defaults to <code>true</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-type">number</span>
        <br>
        <p class="param-description">Set to one of the <code>RepositoryInit</code> constants, or a custom value.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noDotgitDir</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Normally a &#39;/.git/&#39; will be appended to the repo path for non-bare repos (if it is not already there), but passing this flag prevents that behavior.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noReinit</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Return an error if the repository path appears to already be a git repository.  Defaults to <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">originUrl</span><span class="param-type">string</span>
        <br>
        <p class="param-description">If set, then after the rest of the repository initialization is completed an <code>origin</code> remote will be added pointing to this URL.</p>
      </li>
      <li class="param-li">
        <span class="param-name">templatePath</span><span class="param-type">string</span>
        <br>
        <p class="param-description">When the <code>externalTemplate</code> option is set, this is the first location to check for the template directory.  If this is not configured, then the default locations will be searched instead.</p>
      </li>
      <li class="param-li">
        <span class="param-name">workdirPath</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The path to the working directory.  If this is a relative path it will be evaluated relative to the repo path. If this is not the &quot;natural&quot; working directory, a .git gitlink file will be created here linking to the repo path.</p>
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
    <p class="param-description">A new repository.</p>
  </li>
</ul>

## Examples

Basic example.

```ts
import { initRepository } from 'es-git';

const repo = await iniRepository('/path/to/repo');
```

Create bare repository.

```ts
import { initRepository } from 'es-git';

const repo = await iniRepository('/path/to/repo.git', {
  bare: true,
});
```
