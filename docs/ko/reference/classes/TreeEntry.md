[es-git](../globals.md) / TreeEntry

# 클래스: TreeEntry

트리 내부의 항목을 나타내는 클래스예요. 이 항목은 트리에서 빌려온 것이에요.

## 메소드

### id()

> **id**(): `string`

항목이 가리키는 개체의 id를 가져와요.

#### 반환 형식:

`string`

***

### name()

> **name**(): `string`

트리 항목의 파일 이름을 가져와요.

이름이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

항목이 가리키는 개체의 타입을 가져와요.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### filemode()

> **filemode**(): `number`

트리 항목의 UNIX 파일 속성을 가져와요.

#### 반환 형식:

`number`

***

### toObject()

> **toObject**(`repo`): [`GitObject`](GitObject.md)

트리 항목을 해당 항목이 가리키는 Git 개체로 변환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `repo` | [`Repository`](Repository.md) |

#### 반환 형식:

[`GitObject`](GitObject.md)
