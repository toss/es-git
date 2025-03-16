# findReference

Lookup a reference to one of the objects in a repository.

## Signature

```ts
class Repository {
  findReference(name: string): Reference | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Reference name to lookup.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Reference</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if the reference does not exist.</p>
  </li>
</ul>

## Examples

Get `HEAD` reference from the repository.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const reference = repo.findReference('HEAD');
```