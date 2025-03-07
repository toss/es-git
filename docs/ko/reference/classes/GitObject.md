[es-git](../globals.md) / GitObject

# 클래스: GitObject

A structure to represent a git [object][1]

[1]: http://git-scm.com/book/en/Git-Internals-Git-Objects

## 메소드

### id()

> **id**(): `string`

Get the id (SHA1) of a repository object

#### 반환 형식:

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the object type of object.

If the type is unknown, then `null` is returned.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### peel()

> **peel**(`objType`): [`GitObject`](GitObject.md)

Recursively peel an object until an object of the specified type is met.

If you pass `Any` as the target type, then the object will be
peeled until the type changes (e.g. a tag will be chased until the
referenced object is no longer a tag).

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### peelToCommit()

> **peelToCommit**(): [`Commit`](Commit.md)

Recursively peel an object until a commit is found

#### 반환 형식:

[`Commit`](Commit.md)

***

### peelToBlob()

> **peelToBlob**(): [`Blob`](Blob.md)

Recursively peel an object until a blob is found

#### 반환 형식:

[`Blob`](Blob.md)

***

### asCommit()

> **asCommit**(): `null` \| [`Commit`](Commit.md)

Attempt to view this object as a commit.

Returns `null` if the object is not actually a commit.

#### 반환 형식:

`null` \| [`Commit`](Commit.md)
