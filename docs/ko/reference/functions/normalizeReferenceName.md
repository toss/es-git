[es-git](../globals.md) / normalizeReferenceName

# 함수: normalizeReferenceName()

> **normalizeReferenceName**(`refname`, `format`?): `string` \| `null`

Normalize reference name and check validity.

This will normalize the reference name by collapsing runs of adjacent
slashes between name components into a single slash. It also validates
the name according to the following rules:

1. If `ReferenceFormat.AllowOnelevel` is given, the name may
   contain only capital letters and underscores, and must begin and end
   with a letter. (e.g. "HEAD", "ORIG_HEAD").
2. The flag `ReferenceFormat.RefspecShorthand` has an effect
   only when combined with `ReferenceFormat.AllowOnelevel`. If
   it is given, "shorthand" branch names (i.e. those not prefixed by
   `refs/`, but consisting of a single word without `/` separators)
   become valid. For example, "main" would be accepted.
3. If `ReferenceFormat.RefspecPattern` is given, the name may
   contain a single `*` in place of a full pathname component (e.g.
   `foo/*/bar`, `foo/bar*`).
4. Names prefixed with "refs/" can be almost anything. You must avoid
   the characters '~', '^', ':', '\\', '?', '[', and '*', and the
   sequences ".." and "@{" which have special meaning to revparse.

If the reference passes validation, it is returned in normalized form,
otherwise an `null` is returned.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refname` | `string` |
| `format`? | `null` \| `number` |

## 반환 형식:

`string` \| `null`

## Example

```ts
import { normalizeReferenceName, ReferenceFormat } from 'es-git';

console.assert(
  normalizeReferenceName('foo//bar"),
  'foo/bar'
);
console.assert(
  normalizeReferenceName(
    'HEAD',
    ReferenceFormat.AllowOnelevel
  ),
  'HEAD'
);
console.assert(
  normalizeReferenceName(
    'refs/heads/*',
    ReferenceFormat.RefspecPattern
  ),
  'refs/heads/*'
);
console.assert(
  normalizeReferenceName(
    'main',
    ReferenceFormat.AllowOnelevel | ReferenceFormat.RefspecShorthand
  ),
  'main'
);
```
