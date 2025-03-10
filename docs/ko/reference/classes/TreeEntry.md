[es-git](../globals.md) / TreeEntry

# 클래스: TreeEntry

A class representing an entry inside of a tree. An entry is borrowed
from a tree.

## 메소드

### id()

> **id**(): `string`

Get the id of the object pointed by the entry.

#### 반환 형식:

`string`

***

### name()

> **name**(): `string`

Get the filename of a tree entry.

Throws error if the name is not valid utf-8.

#### 반환 형식:

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the type of the object pointed by the entry.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### filemode()

> **filemode**(): `number`

Get the UNIX file attributes of a tree entry.

#### 반환 형식:

`number`

***

### toObject()

> **toObject**(`repo`): [`GitObject`](GitObject.md)

Convert a tree entry to the object it points to.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `repo` | [`Repository`](Repository.md) |

#### 반환 형식:

[`GitObject`](GitObject.md)
