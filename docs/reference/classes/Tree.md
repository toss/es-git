[es-git](../globals.md) / Tree

# Class: Tree

A class to represent a git [tree][1].

[1]: https://git-scm.com/book/en/Git-Internals-Git-Objects

## Methods

### id()

> **id**(): `string`

Get the id (SHA1) of a repository object.

#### Returns

`string`

***

### len()

> **len**(): `bigint`

Get the number of entries listed in this tree.

#### Returns

`bigint`

***

### isEmpty()

> **isEmpty**(): `boolean`

Return `true` if there is no entry.

#### Returns

`boolean`

***

### iter()

> **iter**(): [`TreeIter`](TreeIter.md)

Returns an iterator over the entries in this tree.

#### Returns

[`TreeIter`](TreeIter.md)

***

### walk()

> **walk**(`mode`, `callback`): `void`

Traverse the entries in a tree and its subtrees in post or pre-order.
The callback function will be run on each node of the tree that's
walked. The return code of this function will determine how the walk
continues.

libgit2 requires that the callback be an integer, where 0 indicates a
successful visit, 1 skips the node, and -1 aborts the traversal completely.
See [libgit2 documentation][1] for more information.

[1]: https://libgit2.org/libgit2/#HEAD/group/tree/git_tree_walk

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `mode` | [`TreeWalkMode`](../type-aliases/TreeWalkMode.md) |
| `callback` | (`entry`) => `number` |

#### Returns

`void`

***

### getId()

> **getId**(`id`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by SHA value.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `id` | `string` |

#### Returns

`null` \| [`TreeEntry`](TreeEntry.md)

***

### get()

> **get**(`index`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by its position in the tree.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `index` | `number` |

#### Returns

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getName()

> **getName**(`filename`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by its filename.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `filename` | `string` |

#### Returns

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getPath()

> **getPath**(`path`): `null` \| [`TreeEntry`](TreeEntry.md)

Retrieve a tree entry contained in a tree or in any of its subtrees,
given its relative path.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |

#### Returns

`null` \| [`TreeEntry`](TreeEntry.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

Casts this Tree to be usable as an `GitObject`.

#### Returns

[`GitObject`](GitObject.md)
