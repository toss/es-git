[es-git](../globals.md) / isReferenceNameValid

# 함수: isReferenceNameValid()

> **isReferenceNameValid**(`refname`): `boolean`

Ensure the reference name is well-formed.

Validation is performed as if [`ReferenceFormat::ALLOW_ONELEVEL`]
was given to [`Reference::normalize_name`]. No normalization is
performed, however.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refname` | `string` |

## 반환 형식:

`boolean`

## Example

```ts
import { isReferenceNameValid } from 'es-git';

console.assert(isReferenceNameValid("HEAD"));
console.assert(isReferenceNameValid("refs/heads/main"));

// But:
console.assert(!isReferenceNameValid("main"));
console.assert(!isReferenceNameValid("refs/heads/*"));
console.assert(!isReferenceNameValid("foo//bar"));
```
