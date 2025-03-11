[es-git](../globals.md) / isValidTagName

# 함수: isValidTagName()

> **isValidTagName**(`tagName`): `boolean`

주어진 태그명(`tagName`)이 올바른 형식인지 확인해요.

이 함수는 태그 이름이 `refs/tags/`가 접두어로 붙어서 유효한 레퍼런스명인지, 그리고 태그 이름이 유효한지 (예: 이름이 `-`로 시작할 수 없어요) 확인해요.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `tagName` | `string` |

## 반환 형식:

`boolean`
