# remoteNames

List all remotes for a given repository

## Signature

```ts
class Repository {
  remoteNames(): string[];
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">All remote names for this repository.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
console.log(repo.remoteNames()); // ["origin"]
```