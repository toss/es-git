# defaultBranch

Get the remote’s default branch.

The `fetch` operation from the remote is also performed.

## Signature

```ts
class Remote {
  defaultBranch(signal?: AbortSignal | null | undefined): Promise<string>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;string&gt;</span>
    <br>
    <p class="param-description">Default branch name.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

const branch = await remote.defaultBranch();
console.log(branch); // "refs/heads/main"
```
