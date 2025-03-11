[es-git](../globals.md) / hashObjectOid

# 함수: hashObjectOid()

> **hashObjectOid**(`objType`, `bytes`): `string`

주어진 데이터를 지정된 개체 유형(`objType`)으로 해시한 후,  
그 결과에 해당하는 OID(Object ID)를 반환해요.

이 함수는 해시된 개체를 리포지토리나 개체 데이터베이스에 저장하지 않아요.

## 매개변수

| 매개변수      | 유형                                            |
|-----------|-----------------------------------------------|
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `bytes`   | `Buffer`                                      |

## 반환 형식:

`string`
