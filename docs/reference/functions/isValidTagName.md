[es-git](../globals.md) / isValidTagName

# Function: isValidTagName()

> **isValidTagName**(`tagName`): `boolean`

Determine whether a tag name is valid, meaning that (when prefixed with refs/tags/) that
it is a valid reference name, and that any additional tag name restrictions are imposed
(eg, it cannot start with a -).

## Parameters

| Parameter | Type |
| ------ | ------ |
| `tagName` | `string` |

## Returns

`boolean`
