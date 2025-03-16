# isZeroOid

Test if this Oid is all zeros.

## Signature

```ts
function isZeroOid(value: string): boolean;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">String to check if is zero Oid.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  if the string is zero Oid.</p>
  </li>
</ul>

## Examples

```ts
import { zeroOid, isZeroOid } from 'es-git';

console.assert(isZeroOid(zeroOid());
```