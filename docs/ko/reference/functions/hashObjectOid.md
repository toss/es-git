[es-git](../globals.md) / hashObjectOid

# 함수: hashObjectOid()

> **hashObjectOid**(`objType`, `bytes`): `string`

Hashes the provided data as an object of the provided type, and returns
an Oid corresponding to the result. This does not store the object
inside any object database or repository.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `bytes` | `Buffer` |

## 반환 형식:

`string`
