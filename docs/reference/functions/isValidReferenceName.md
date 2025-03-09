[es-git](../globals.md) / isValidReferenceName

# Function: isValidReferenceName()

> **isValidReferenceName**(`refname`): `boolean`

Ensure the reference name is well-formed.

Validation is performed as if `ReferenceFormat.AllowOneLevel`
was given to `normalizeReferenceName`. No normalization is
performed, however.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `refname` | `string` |

## Returns

`boolean`

## Example

```ts
import { isValidReferenceName } from 'es-git';

console.assert(isValidReferenceName("HEAD"));
console.assert(isValidReferenceName("refs/heads/main"));

// But:
console.assert(!isValidReferenceName("main"));
console.assert(!isValidReferenceName("refs/heads/*"));
console.assert(!isValidReferenceName("foo//bar"));
```
