[es-git](../globals.md) / hashFileOid

# Function: hashFileOid()

> **hashFileOid**(`objType`, `path`): `string`

Hashes the content of the provided file as an object of the provided type,
and returns an Oid corresponding to the result. This does not store the object
inside any object database or repository.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `path` | `string` |

## Returns

`string`
