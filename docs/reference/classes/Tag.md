[es-git](../globals.md) / Tag

# Class: Tag

A structure to represent a git [tag][1]

[1]: http://git-scm.com/book/en/Git-Basics-Tagging

## Methods

### id()

> **id**(): `string`

Get the id (SHA1) of a repository tag

#### Returns

`string`

***

### message()

> **message**(): `null` \| `string`

Get the message of a tag

Returns `null`` if there is no message or if it is not valid utf8

#### Returns

`null` \| `string`

***

### name()

> **name**(): `string`

Get the name of a tag

Throws error if it is not valid utf8

#### Returns

`string`

***

### peel()

> **peel**(): [`GitObject`](GitObject.md)

Recursively peel a tag until a non tag git_object is found

#### Returns

[`GitObject`](GitObject.md)

***

### tagger()

> **tagger**(): `null` \| [`Signature`](../interfaces/Signature.md)

Get the tagger (author) of a tag

If the author is unspecified, then `null` is returned.

#### Returns

`null` \| [`Signature`](../interfaces/Signature.md)

***

### target()

> **target**(): [`GitObject`](GitObject.md)

Get the tagged object of a tag

This method performs a repository lookup for the given object and
returns it

#### Returns

[`GitObject`](GitObject.md)

***

### targetId()

> **targetId**(): `string`

Get the OID of the tagged object of a tag

#### Returns

`string`

***

### targetType()

> **targetType**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

Get the ObjectType of the tagged object of a tag

#### Returns

`null` \| [`ObjectType`](../enumerations/ObjectType.md)
