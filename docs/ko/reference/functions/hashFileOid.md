[es-git](../globals.md) / hashFileOid

# 함수: hashFileOid()

> **hashFileOid**(`objType`, `path`): `string`

Hashes the content of the provided file as an object of the provided type,
and returns an Oid corresponding to the result. This does not store the object
inside any object database or repository.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `path` | `string` |

## 반환 형식:

`string`
