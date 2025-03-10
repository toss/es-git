[es-git](../globals.md) / Index

# 클래스: Index

A class to represent a git [index][1].

[1]: https://git-scm.com/book/en/Git-Internals-Git-Objects

## 메소드

### version()

> **version**(): `number`

Get index on-disk version.

Valid return values are 2, 3, or 4. If 3 is returned, an index
with version 2 may be written instead, if the extension data in
version 3 is not necessary.

#### 반환 형식:

`number`

***

### setVersion()

> **setVersion**(`version`): `void`

Set index on-disk version.

Valid values are 2, 3, or 4. If 2 is given, git_index_write may
write an index with version 3 instead, if necessary to accurately
represent the index.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `version` | `number` |

#### 반환 형식:

`void`

***

### getByPath()

> **getByPath**(`path`, `stage`?): `null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

Get one of the entries in the index by its path.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |
| `stage`? | `null` \| [`IndexStage`](../type-aliases/IndexStage.md) |

#### 반환 형식:

`null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

***

### addPath()

> **addPath**(`path`): `void`

Add or update an index entry from a file on disk.

The file path must be relative to the repository's working folder and
must be readable.

This method will fail in bare index instances.

This forces the file to be added to the index, not looking at gitignore
rules.

If this file currently is the result of a merge conflict, this file will
no longer be marked as conflicting. The data about the conflict will be
moved to the "resolve undo" (REUC) section.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |

#### 반환 형식:

`void`

***

### addAll()

> **addAll**(`pathspecs`, `options`?): `void`

Add or update index entries matching files in the working directory.

This method will fail in bare index instances.

The `pathspecs` are a list of file names or shell glob patterns that
will matched against files in the repository's working directory. Each
file that matches will be added to the index (either updating an
existing entry or adding a new entry).

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexAddAllOptions`](../interfaces/IndexAddAllOptions.md) |

#### 반환 형식:

`void`

#### Example

Emulate `git add *`:

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
index.addAll(['*']);
index.write();
```

***

### read()

> **read**(`force`?): `void`

Update the contents of an existing index object in memory by reading
from the hard disk.

If force is true, this performs a "hard" read that discards in-memory
changes and always reloads the on-disk index data. If there is no
on-disk version, the index will be cleared.

If force is false, this does a "soft" read that reloads the index data
from disk only if it has changed since the last time it was loaded.
Purely in-memory index data will be untouched. Be aware: if there are
changes on disk, unwritten in-memory changes are discarded.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `force`? | `null` \| `boolean` |

#### 반환 형식:

`void`

***

### write()

> **write**(): `void`

Write an existing index object from memory back to disk using an atomic
file lock.

#### 반환 형식:

`void`

***

### writeTree()

> **writeTree**(): `string`

Write the index as a tree.

This method will scan the index and write a representation of its
current state back to disk; it recursively creates tree objects for each
of the subtrees stored in the index, but only returns the OID of the
root tree. This is the OID that can be used e.g. to create a commit.

The index instance cannot be bare, and needs to be associated to an
existing repository.

The index must not contain any file in conflict.

#### 반환 형식:

`string`

***

### removePath()

> **removePath**(`path`, `options`?): `void`

Remove an index entry corresponding to a file on disk.

The file path must be relative to the repository's working folder. It
may exist.

If this file currently is the result of a merge conflict, this file will
no longer be marked as conflicting. The data about the conflict will be
moved to the "resolve undo" (REUC) section.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |
| `options`? | `null` \| [`IndexRemoveOptions`](../interfaces/IndexRemoveOptions.md) |

#### 반환 형식:

`void`

***

### removeAll()

> **removeAll**(`pathspecs`, `options`?): `void`

Remove all matching index entries.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexRemoveAllOptions`](../interfaces/IndexRemoveAllOptions.md) |

#### 반환 형식:

`void`

***

### updateAll()

> **updateAll**(`pathspecs`, `options`?): `void`

Update all index entries to match the working directory.

This method will fail in bare index instances.

This scans the existing index entries and synchronizes them with the
working directory, deleting them if the corresponding working directory
file no longer exists otherwise updating the information (including
adding the latest version of file to the ODB if needed).

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexUpdateAllOptions`](../interfaces/IndexUpdateAllOptions.md) |

#### 반환 형식:

`void`

***

### count()

> **count**(): `number`

Get the count of entries currently in the index.

#### 반환 형식:

`number`

***

### isEmpty()

> **isEmpty**(): `boolean`

Return `true` is there is no entry in the index.

#### 반환 형식:

`boolean`

***

### path()

> **path**(): `null` \| `string`

Get the full path to the index file on disk.

Returns `None` if this is an in-memory index.

#### 반환 형식:

`null` \| `string`

***

### hasConflicts()

> **hasConflicts**(): `boolean`

Does this index have conflicts?

Returns `true` if the index contains conflicts, `false` if it does not.

#### 반환 형식:

`boolean`

***

### entries()

> **entries**(): [`IndexEntries`](IndexEntries.md)

Get an iterator over the entries in this index.

#### 반환 형식:

[`IndexEntries`](IndexEntries.md)
