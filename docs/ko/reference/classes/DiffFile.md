[es-git](../globals.md) / DiffFile

# 클래스: DiffFile

Description of one side of a delta.

Although this is called a "file" it could represent a file, a symbolic
link, a submodule commit id, or even a tree (although that only happens if
you are tracking type changes or ignored/untracked directories).

## 메소드

### id()

> **id**(): `string`

Returns the Oid of this item.

If this entry represents an absent side of a diff (e.g. the `old_file`
of a `Added` delta), then the oid returned will be zeroes.

#### 반환 형식:

`string`

***

### path()

> **path**(): `null` \| `string`

Returns the path of the entry relative to the working directory of the
repository.

#### 반환 형식:

`null` \| `string`

***

### size()

> **size**(): `bigint`

Returns the size of this entry, in bytes

#### 반환 형식:

`bigint`

***

### isBinary()

> **isBinary**(): `boolean`

Returns `true` if file(s) are treated as binary data.

#### 반환 형식:

`boolean`

***

### isValidId()

> **isValidId**(): `boolean`

Returns `true` if `id` value is known correct.

#### 반환 형식:

`boolean`

***

### exists()

> **exists**(): `boolean`

Returns `true` if file exists at this side of the delta.

#### 반환 형식:

`boolean`

***

### mode()

> **mode**(): [`FileMode`](../type-aliases/FileMode.md)

Returns file mode.

#### 반환 형식:

[`FileMode`](../type-aliases/FileMode.md)
