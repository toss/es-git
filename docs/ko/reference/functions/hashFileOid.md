[es-git](../globals.md) / hashFileOid

# 함수: hashFileOid()

> **hashFileOid**(`objType`, `path`): `string`

주어진 파일의 내용을 지정된 개체 유형(`objType`)으로 해시(hash)한 후,  
그 결과에 해당하는 OID(Object ID)를 반환해요.

이 함수는 해시된 개체를 리포지토리나 개체 데이터베이스에 저장하지 않아요.

## 매개변수

| 매개변수      | 유형                                            |
|-----------|-----------------------------------------------|
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |
| `path`    | `string`                                      |

## 반환 형식:

`string`
