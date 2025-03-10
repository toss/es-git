[es-git](../globals.md) / DiffFile

# Class: DiffFile

Description of one side of a delta.

Although this is called a "file" it could represent a file, a symbolic
link, a submodule commit id, or even a tree (although that only happens if
you are tracking type changes or ignored/untracked directories).

## Methods

### id()

> **id**(): `string`

Returns the Oid of this item.

If this entry represents an absent side of a diff (e.g. the `oldFile`
of a `Added` delta), then the oid returned will be zeroes.

#### Returns

`string`

***

### path()

> **path**(): `null` \| `string`

Returns the path of the entry relative to the working directory of the
repository.

#### Returns

`null` \| `string`

***

### size()

> **size**(): `bigint`

Returns the size of this entry, in bytes.

#### Returns

`bigint`

***

### isBinary()

> **isBinary**(): `boolean`

Returns `true` if file(s) are treated as binary data.

#### Returns

`boolean`

***

### isValidId()

> **isValidId**(): `boolean`

Returns `true` if `id` value is known correct.

#### Returns

`boolean`

***

### exists()

> **exists**(): `boolean`

Returns `true` if file exists at this side of the delta.

#### Returns

`boolean`

***

### mode()

> **mode**(): [`FileMode`](../type-aliases/FileMode.md)

Returns file mode.

#### Returns

[`FileMode`](../type-aliases/FileMode.md)
