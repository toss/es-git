[es-git](../globals.md) / isValidTagName

# 함수: isValidTagName()

> **isValidTagName**(`tagName`): `boolean`

Determine whether a tag name is valid, meaning that (when prefixed with refs/tags/) that
it is a valid reference name, and that any additional tag name restrictions are imposed
(eg, it cannot start with a -).

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `tagName` | `string` |

## 반환 형식:

`boolean`
