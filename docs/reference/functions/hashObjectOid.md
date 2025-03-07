[es-git](../globals.md) / hashObjectOid

# Function: hashObjectOid()

> **hashObjectOid**(`objType`, `bytes`): `string`

Hashes the provided data as an object of the provided type, and returns
an Oid corresponding to the result. This does not store the object
inside any object database or repository.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `bytes` | `Buffer` |

## Returns

`string`
