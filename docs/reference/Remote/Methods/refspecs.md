# refspecs

List all refspecs.

Filter refspec if has not valid `src` or `dst` with utf-8.

## Signature

```ts
class Remote {
  refspecs(): Refspec[];
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Refspec[]</span>
    <br>
    <p class="param-description">List all refspecs.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// Retrieving the Refspecs configured for this remote
const refspecs = remote.refspecs();
console.log(refspecs[0]);
// For the "+refs/heads/*:refs/remotes/origin/*" Refspec
// {
//   "direction": "Fetch",
//   "src": "refs/heads/*",
//   "dst": "refs/remotes/origin/*",
//   "force": true
// }
```