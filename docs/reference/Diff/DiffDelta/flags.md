# flags

Returns the flags on the delta.

## Signature

```ts
class DiffDelta {
  flags(): number;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">The flags on the delta.</p>
  </li>
</ul>

## Examples

```ts
import { DiffDelta, DiffFlags, diffFlagsContains } from 'es-git';

const delta: DiffDelta;
console.assert(diffFlagsContains(delta.flags(), DiffFlags.Binary | DiffFlags.ValidId));
```