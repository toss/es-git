# getReference

Lookup a reference to one of the objects in a repository.

## Signature

```ts
class Repository {
  getReference(name: string): Reference;
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
    <span class="param-type">Reference</span>
    <br>
    <p class="param-description">Git reference.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the reference does not exist.</p>
  </li>
</ul>

## Examples

Get `HEAD` reference from the repository.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const reference = repo.getReference('HEAD');
```