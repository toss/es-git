[es-git](../globals.md) / isValidReferenceName

# 함수: isValidReferenceName()

> **isValidReferenceName**(`refname`): `boolean`

주어진 레퍼런스명(`refname`)이 올바른 형식인지 확인해요.

이 검사는 `normalizeReferenceName` 함수에 `ReferenceFormat.AllowOnelevel` 옵션을 적용한 것처럼 동작하지만, 실제로 레퍼런스 이름을 변환하지는 않아요.

## 매개변수

| 매개변수      | 유형       |
|-----------|----------|
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
