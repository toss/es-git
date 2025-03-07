[es-git](../globals.md) / TreeEntry

# Class: TreeEntry

A structure representing an entry inside of a tree. An entry is borrowed
from a tree.

## Methods

### id()

> **id**(): `string`

Get the id of the object pointed by the entry

#### Returns

`string`

***

### name()

> **name**(): `string`

Get the filename of a tree entry

Throws error if the name is not valid utf-8

#### Returns

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the type of the object pointed by the entry

#### Returns

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### filemode()

> **filemode**(): `number`

Get the UNIX file attributes of a tree entry

#### Returns

`number`

***

### toObject()

> **toObject**(`repo`): [`GitObject`](GitObject.md)

Convert a tree entry to the object it points to.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `repo` | [`Repository`](Repository.md) |

#### Returns

[`GitObject`](GitObject.md)
