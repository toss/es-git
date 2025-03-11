[es-git](../globals.md) / normalizeReferenceName

# 함수: normalizeReferenceName()

> **normalizeReferenceName**(`refname`, `format`?): `string` \| `null`

주어진 레퍼런스 이름(`refname`)을 정규화하고 유효한지 확인해요.
이 함수는 이름의 구성 요소들 사이에서 연속적으로 나타나는 슬래시(`/`)를 하나의 슬래시로 축소하여 레퍼런스 이름을 정규화하며, 아래 규칙에 따라 이름을 검증해요:
1. `ReferenceFormat.AllowOnelevel` 옵션이 주어지면, 레퍼런스 이름에는 대문자와 밑줄만 사용할 수 있으며, 반드시 문자로 시작하고 끝나야 해요. (예: `"HEAD"`, `"ORIG_HEAD"`)
2. `ReferenceFormat.RefspecShorthand` 플래그는 `ReferenceFormat.AllowOnelevel`과 결합될 때만 효과가 있어요. 이 경우, 슬래시(`/`)가 포함되지 않은 단어 하나로 구성된 "축약형" 브랜치 이름(예: `"main"`)도 유효해요.
3. `ReferenceFormat.RefspecPattern` 옵션이 주어지면, 레퍼런스 이름에는 전체 경로 구성 요소 대신 단일 `*`를 포함할 수 있어요. (예: `"foo/*/bar"`, `"foo/bar*"`)
4. `refs/`로 시작하는 이름에게는 대부분의 규칙이 적용되지 않아요. 다만, `~`, `^`, `:`, `\`, `?`, `[`, `*` 문자들과 `..`, `@{`의 시퀀스는 사용 불가능해요. 이 문자는 `revparse`에서 특별한 의미를 가지기 때문이에요.

레퍼런스가 검증을 통과하면 정규화된 형태로 반환되며, 그렇지 않은 경우 `null`을 반환해요.

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
  normalizeReferenceName('foo//bar'),
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
