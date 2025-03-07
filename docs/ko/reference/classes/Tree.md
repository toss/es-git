[es-git](../globals.md) / Tree

# 클래스: Tree

A structure to represent a git [tree][1]

[1]: http://git-scm.com/book/en/Git-Internals-Git-Objects

## 메소드

### id()

> **id**(): `string`

Get the id (SHA1) of a repository object

#### 반환 형식:

`string`

***

### len()

> **len**(): `bigint`

Get the number of entries listed in this tree.

#### 반환 형식:

`bigint`

***

### isEmpty()

> **isEmpty**(): `boolean`

Return `true` if there is not entry

#### 반환 형식:

`boolean`

***

### iter()

> **iter**(): [`TreeIter`](TreeIter.md)

Returns an iterator over the entries in this tree.

#### 반환 형식:

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

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `mode` | [`TreeWalkMode`](../type-aliases/TreeWalkMode.md) |
| `callback` | (`entry`) => `number` |

#### 반환 형식:

`void`

***

### getId()

> **getId**(`id`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by SHA value.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `id` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### get()

> **get**(`index`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by its position in the tree

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `index` | `number` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getName()

> **getName**(`filename`): `null` \| [`TreeEntry`](TreeEntry.md)

Lookup a tree entry by its filename

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `filename` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getPath()

> **getPath**(`path`): `null` \| [`TreeEntry`](TreeEntry.md)

Retrieve a tree entry contained in a tree or in any of its subtrees,
given its relative path.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

Casts this Tree to be usable as an `GitObject`

#### 반환 형식:

[`GitObject`](GitObject.md)
