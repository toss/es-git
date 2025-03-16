# isValidReferenceName

Ensure the reference name is well-formed.

Validation is performed as if `ReferenceFormat.AllowOnelevel`
was given to `normalizeReferenceName`  No normalization is performed, however.

## Signature

```ts
function isValidReferenceName(refname: string): boolean;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Reference name to check if it is valid.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  if reference name is valid.</p>
  </li>
</ul>

## Examples

```ts
import { isValidReferenceName } from 'es-git';

console.assert(isValidReferenceName("HEAD"));
console.assert(isValidReferenceName("refs/heads/main"));

// But:
console.assert(!isValidReferenceName("main"));
console.assert(!isValidReferenceName("refs/heads/*"));
console.assert(!isValidReferenceName("foo//bar"));
```