[es-git](../globals.md) / Tag

# 클래스: Tag

A structure to represent a git [tag][1]

[1]: http://git-scm.com/book/en/Git-Basics-Tagging

## 메소드

### id()

> **id**(): `string`

Get the id (SHA1) of a repository tag

#### 반환 형식:

`string`

***

### message()

> **message**(): `null` \| `string`

Get the message of a tag

Returns `null`` if there is no message or if it is not valid utf8

#### 반환 형식:

`null` \| `string`

***

### name()

> **name**(): `string`

Get the name of a tag

Throws error if it is not valid utf8

#### 반환 형식:

`string`

***

### peel()

> **peel**(): [`GitObject`](GitObject.md)

Recursively peel a tag until a non tag git_object is found

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### tagger()

> **tagger**(): `null` \| [`Signature`](../interfaces/Signature.md)

Get the tagger (author) of a tag

If the author is unspecified, then `null` is returned.

#### 반환 형식:

`null` \| [`Signature`](../interfaces/Signature.md)

***

### target()

> **target**(): [`GitObject`](GitObject.md)

Get the tagged object of a tag

This method performs a repository lookup for the given object and
returns it

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### targetId()

> **targetId**(): `string`

Get the OID of the tagged object of a tag

#### 반환 형식:

`string`

***

### targetType()

> **targetType**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the ObjectType of the tagged object of a tag

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)
