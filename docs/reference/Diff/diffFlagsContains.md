# diffFlagsContains

Check diff flags contains given flags.

## Signature

```ts
function diffFlagsContains(source: number, target: number): boolean;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">source</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Source flags.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Target flags.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  is source flags contains target flags.</p>
  </li>
</ul>

## Examples

```ts
import { DiffDelta, DiffFlags, diffFlagsContains } from 'es-git';

const delta: DiffDelta;
console.assert(diffFlagsContains(delta.flags(), DiffFlags.Binary | DiffFlags.ValidId));
```