[es-git](../globals.md) / GitObject

# Class: GitObject

A structure to represent a Git [Object][1]

[1]: http://git-scm.com/book/en/Git-Internals-Git-Objects

## Methods

### id()

> **id**(): `string`

Get the id (SHA1) of a repository object

#### Returns

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the object type of object.

If the type is unknown, then `null` is returned.

#### Returns

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### peel()

> **peel**(`objType`): [`GitObject`](GitObject.md)

Recursively peel an object until an object of the specified type is met.

If you pass `Any` as the target type, then the object will be
peeled until the type changes (e.g. a tag will be chased until the
referenced object is no longer a tag).

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |

#### Returns

[`GitObject`](GitObject.md)

***

### peelToCommit()

> **peelToCommit**(): [`Commit`](Commit.md)

Recursively peel an object until a commit is found

#### Returns

[`Commit`](Commit.md)

***

### peelToBlob()

> **peelToBlob**(): [`Blob`](Blob.md)

Recursively peel an object until a blob is found

#### Returns

[`Blob`](Blob.md)

***

### asCommit()

> **asCommit**(): `null` \| [`Commit`](Commit.md)

Attempt to view this object as a commit.

Returns `null` if the object is not actually a commit.

#### Returns

`null` \| [`Commit`](Commit.md)
