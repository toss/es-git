[es-git](../globals.md) / Blob

# Class: Blob

A structure to represent a git [blob][1]

[1]: https://git-scm.com/book/en/Git-Internals-Git-Objects

## Methods

### id()

> **id**(): `string`

Get the id (SHA1) of a repository blob

#### Returns

`string`

***

### isBinary()

> **isBinary**(): `boolean`

Determine if the blob content is most certainly binary or not.

#### Returns

`boolean`

***

### content()

> **content**(): `Uint8Array`

Get the content of this blob.

#### Returns

`Uint8Array`

***

### size()

> **size**(): `bigint`

Get the size in bytes of the contents of this blob.

#### Returns

`bigint`
