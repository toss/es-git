[es-git](../globals.md) / DiffDelta

# 클래스: DiffDelta

Description of changes to one entry.

## 메소드

### flags()

> **flags**(): `number`

Returns the flags on the delta.

For more information, see `DiffFlags`'s documentation.

#### 반환 형식:

`number`

***

### numFiles()

> **numFiles**(): `number`

Returns the number of files in this delta.

#### 반환 형식:

`number`

***

### status()

> **status**(): [`DeltaType`](../type-aliases/DeltaType.md)

Returns the status of this entry

#### 반환 형식:

[`DeltaType`](../type-aliases/DeltaType.md)

***

### oldFile()

> **oldFile**(): [`DiffFile`](DiffFile.md)

Return the file which represents the "from" side of the diff.

What side this means depends on the function that was used to generate
the diff and will be documented on the function itself.

#### 반환 형식:

[`DiffFile`](DiffFile.md)

***

### newFile()

> **newFile**(): [`DiffFile`](DiffFile.md)

Return the file which represents the "to" side of the diff.

What side this means depends on the function that was used to generate
the diff and will be documented on the function itself.

#### 반환 형식:

[`DiffFile`](DiffFile.md)
