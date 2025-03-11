[es-git](../globals.md) / Index

# Class: Index

A class to represent a git [index][1].

[1]: https://git-scm.com/book/en/Git-Internals-Git-Objects

## Methods

### version()

> **version**(): `number`

Get index on-disk version.

Valid return values are 2, 3, or 4. If 3 is returned, an index
with version 2 may be written instead, if the extension data in
version 3 is not necessary.

#### Returns

`number`

***

### setVersion()

> **setVersion**(`version`): `void`

Set index on-disk version.

Valid values are 2, 3, or 4. If 2 is given, git_index_write may
write an index with version 3 instead, if necessary to accurately
represent the index.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `version` | `number` |

#### Returns

`void`

***

### getByPath()

> **getByPath**(`path`, `stage`?): `null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

Get one of the entries in the index by its path.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |
| `stage`? | `null` \| [`IndexStage`](../type-aliases/IndexStage.md) |

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexAddAllOptions`](../interfaces/IndexAddAllOptions.md) |

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `force`? | `null` \| `boolean` |

#### Returns

`void`

***

### write()

> **write**(): `void`

Write an existing index object from memory back to disk using an atomic
file lock.

#### Returns

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

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |
| `options`? | `null` \| [`IndexRemoveOptions`](../interfaces/IndexRemoveOptions.md) |

#### Returns

`void`

***

### removeAll()

> **removeAll**(`pathspecs`, `options`?): `void`

Remove all matching index entries.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexRemoveAllOptions`](../interfaces/IndexRemoveAllOptions.md) |

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexUpdateAllOptions`](../interfaces/IndexUpdateAllOptions.md) |

#### Returns

`void`

***

### count()

> **count**(): `number`

Get the count of entries currently in the index.

#### Returns

`number`

***

### isEmpty()

> **isEmpty**(): `boolean`

Return `true` is there is no entry in the index.

#### Returns

`boolean`

***

### path()

> **path**(): `null` \| `string`

Get the full path to the index file on disk.

Returns `null` if this is an in-memory index.

#### Returns

`null` \| `string`

***

### hasConflicts()

> **hasConflicts**(): `boolean`

Does this index have conflicts?

Returns `true` if the index contains conflicts, `false` if it does not.

#### Returns

`boolean`

***

### entries()

> **entries**(): [`IndexEntries`](IndexEntries.md)

Get an iterator over the entries in this index.

#### Returns

[`IndexEntries`](IndexEntries.md)
