[es-git](../globals.md) / isValidReferenceName

# 함수: isValidReferenceName()

> **isValidReferenceName**(`refname`): `boolean`

Ensure the reference name is well-formed.

Validation is performed as if `ReferenceFormat.AllowOnelevel`
was given to `normalizeReferenceName`
No normalization is performed, however.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refname` | `string` |

## 반환 형식:

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
