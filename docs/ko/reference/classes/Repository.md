[es-git](../globals.md) / Repository

# 클래스: Repository

An owned git repository, representing all state associated with the
underlying filesystem.

This structure corresponds to a Git Repository in libgit2.

When a repository goes out of scope, it is freed in memory but not deleted
from the filesystem.

## 메소드

### findCommit()

> **findCommit**(`oid`): `null` \| [`Commit`](Commit.md)

Lookup a reference to one of the commits in a repository.

Returns `null` if the commit does not exist.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Commit`](Commit.md)

***

### getCommit()

> **getCommit**(`oid`): [`Commit`](Commit.md)

Lookup a reference to one of the commits in a repository.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Commit`](Commit.md)

***

### commit()

> **commit**(`tree`, `message`, `options`?): `string`

Create new commit in the repository

If the `updateRef` is not `null`, name of the reference that will be
updated to point to this commit. If the reference is not direct, it will
be resolved to a direct reference. Use "HEAD" to update the HEAD of the
current branch and make it point to this commit. If the reference
doesn't exist yet, it will be created. If it does exist, the first
parent must be the tip of this branch.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `tree` | [`Tree`](Tree.md) |
| `message` | `string` |
| `options`? | `null` \| [`CommitOptions`](../interfaces/CommitOptions.md) |

#### 반환 형식:

`string`

***

### diffTreeToTree()

> **diffTreeToTree**(`oldTree`?, `newTree`?, `options`?): [`Diff`](Diff.md)

Create a diff with the difference between two tree objects.

This is equivalent to `git diff <old-tree> <new-tree>`

The first tree will be used for the "old_file" side of the delta and the
second tree will be used for the "new_file" side of the delta.  You can
pass `null` to indicate an empty tree, although it is an error to pass
`null` for both the `oldTree` and `newTree`.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oldTree`? | `null` \| [`Tree`](Tree.md) |
| `newTree`? | `null` \| [`Tree`](Tree.md) |
| `options`? | `null` \| [`DiffOptions`](../interfaces/DiffOptions.md) |

#### 반환 형식:

[`Diff`](Diff.md)

***

### diffIndexToIndex()

> **diffIndexToIndex**(`oldIndex`, `newIndex`, `options`?): [`Diff`](Diff.md)

Create a diff between two index objects.

The first index will be used for the "oldFile" side of the delta, and
the second index will be used for the "newFile" side of the delta.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oldIndex` | [`Index`](Index.md) |
| `newIndex` | [`Index`](Index.md) |
| `options`? | `null` \| [`DiffOptions`](../interfaces/DiffOptions.md) |

#### 반환 형식:

[`Diff`](Diff.md)

***

### diffIndexToWorkdir()

> **diffIndexToWorkdir**(`index`?, `options`?): [`Diff`](Diff.md)

Create a diff between the repository index and the workdir directory.

This matches the `git diff` command.  See the note below on
`treeToWorkdir` for a discussion of the difference between
`git diff` and `git diff HEAD` and how to emulate a `git diff <treeish>`
using libgit2.

The index will be used for the "oldFile" side of the delta, and the
working directory will be used for the "newFile" side of the delta.

If you pass `null` for the index, then the existing index of the `repo`
will be used. In this case, the index will be refreshed from disk
(if it has changed) before the diff is generated.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `index`? | `null` \| [`Index`](Index.md) |
| `options`? | `null` \| [`DiffOptions`](../interfaces/DiffOptions.md) |

#### 반환 형식:

[`Diff`](Diff.md)

***

### diffTreeToWorkdir()

> **diffTreeToWorkdir**(`oldTree`?, `options`?): [`Diff`](Diff.md)

Create a diff between a tree and the working directory.

The tree you provide will be used for the "oldFile" side of the delta,
and the working directory will be used for the "newFile" side.

This is not the same as `git diff <treeish>` or `git diff-index <treeish>`.
Those commands use information from the index, whereas this
function strictly returns the differences between the tree and the files
in the working directory, regardless of the state of the index.  Use
`treeToWorkdirWithIndex` to emulate those commands.

To see difference between this and `treeToWorkdirWithIndex`,
consider the example of a staged file deletion where the file has then
been put back into the working dir and further modified.  The
tree-to-workdir diff for that file is 'modified', but `git diff` would
show status 'deleted' since there is a staged delete.

If `null` is passed for `tree`, then an empty tree is used.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oldTree`? | `null` \| [`Tree`](Tree.md) |
| `options`? | `null` \| [`DiffOptions`](../interfaces/DiffOptions.md) |

#### 반환 형식:

[`Diff`](Diff.md)

***

### diffTreeToWorkdirWithIndex()

> **diffTreeToWorkdirWithIndex**(`oldTree`?, `options`?): [`Diff`](Diff.md)

Create a diff between a tree and the working directory using index data
to account for staged deletes, tracked files, etc.

This emulates `git diff <tree>` by diffing the tree to the index and
the index to the working directory and blending the results into a
single diff that includes staged deleted, etc.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oldTree`? | `null` \| [`Tree`](Tree.md) |
| `options`? | `null` \| [`DiffOptions`](../interfaces/DiffOptions.md) |

#### 반환 형식:

[`Diff`](Diff.md)

***

### index()

> **index**(): [`Index`](Index.md)

Get the Index file for this repository.

If a custom index has not been set, the default index for the repository
will be returned (the one located in .git/index).

#### 반환 형식:

[`Index`](Index.md)

***

### findObject()

> **findObject**(`oid`): `null` \| [`GitObject`](GitObject.md)

Lookup a reference to one of the objects in a repository.

Returns `null` if the object does not exist.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`GitObject`](GitObject.md)

***

### getObject()

> **getObject**(`oid`): [`GitObject`](GitObject.md)

Lookup a reference to one of the objects in a repository.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### findReference()

> **findReference**(`name`): `null` \| [`Reference`](Reference.md)

Lookup a reference to one of the objects in a repository.

Returns `null` if reference not exists.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`null` \| [`Reference`](Reference.md)

***

### getReference()

> **getReference**(`name`): [`Reference`](Reference.md)

Lookup a reference to one of the objects in a repository.

Throws error if reference not exists.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

[`Reference`](Reference.md)

***

### remoteNames()

> **remoteNames**(): `string`[]

List all remotes for a given repository

#### 반환 형식:

`string`[]

***

### getRemote()

> **getRemote**(`name`): [`Remote`](Remote.md)

Get remote from repository

Throws error if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

[`Remote`](Remote.md)

***

### findRemote()

> **findRemote**(`name`): `null` \| [`Remote`](Remote.md)

Find remote from repository

Returns `null` if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`null` \| [`Remote`](Remote.md)

***

### createRemote()

> **createRemote**(`name`, `url`, `options`?): [`Remote`](Remote.md)

Add a remote with the default fetch refspec to the repository’s configuration.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |
| `url` | `string` |
| `options`? | `null` \| [`CreateRemoteOptions`](../interfaces/CreateRemoteOptions.md) |

#### 반환 형식:

[`Remote`](Remote.md)

***

### isBare()

> **isBare**(): `boolean`

Tests whether this repository is a bare repository or not.

#### 반환 형식:

`boolean`

***

### isShallow()

> **isShallow**(): `boolean`

Tests whether this repository is a shallow clone.

#### 반환 형식:

`boolean`

***

### isWorktree()

> **isWorktree**(): `boolean`

Tests whether this repository is a worktree.

#### 반환 형식:

`boolean`

***

### isEmpty()

> **isEmpty**(): `boolean`

Tests whether this repository is empty.

#### 반환 형식:

`boolean`

***

### path()

> **path**(): `string`

Returns the path to the `.git` folder for normal repositories or the
repository itself for bare repositories.

#### 반환 형식:

`string`

***

### state()

> **state**(): [`RepositoryState`](../type-aliases/RepositoryState.md)

Returns the current state of this repository

#### 반환 형식:

[`RepositoryState`](../type-aliases/RepositoryState.md)

***

### workdir()

> **workdir**(): `null` \| `string`

Get the path of the working directory for this repository.

If this repository is bare, then `null` is returned.

#### 반환 형식:

`null` \| `string`

***

### head()

> **head**(): [`Reference`](Reference.md)

Retrieve and resolve the reference pointed at by HEAD.

#### 반환 형식:

[`Reference`](Reference.md)

***

### setHead()

> **setHead**(`refname`): `void`

Make the repository HEAD point to the specified reference.

If the provided reference points to a tree or a blob, the HEAD is
unaltered and an error is returned.

If the provided reference points to a branch, the HEAD will point to
that branch, staying attached, or become attached if it isn't yet. If
the branch doesn't exist yet, no error will be returned. The HEAD will
then be attached to an unborn branch.

Otherwise, the HEAD will be detached and will directly point to the
commit.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refname` | `string` |

#### 반환 형식:

`void`

***

### revparse()

> **revparse**(`spec`): [`Revspec`](../interfaces/Revspec.md)

Execute a rev-parse operation against the `spec` listed.

The resulting revision specification is returned, or an error is
returned if one occurs.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `spec` | `string` |

#### 반환 형식:

[`Revspec`](../interfaces/Revspec.md)

***

### revparseSingle()

> **revparseSingle**(`spec`): `string`

Find a single object, as specified by a revision string.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `spec` | `string` |

#### 반환 형식:

`string`

***

### revwalk()

> **revwalk**(): [`Revwalk`](Revwalk.md)

Create a revwalk that can be used to traverse the commit graph.

#### 반환 형식:

[`Revwalk`](Revwalk.md)

***

### findTag()

> **findTag**(`oid`): `null` \| [`Tag`](Tag.md)

Lookup a tag object by prefix hash from the repository.

Returns `null` if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Tag`](Tag.md)

***

### getTag()

> **getTag**(`oid`): [`Tag`](Tag.md)

Lookup a tag object by prefix hash from the repository.

Throws error if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Tag`](Tag.md)

***

### tagNames()

> **tagNames**(`pattern`?): `string`[]

Get a list with all the tags in the repository.

An optional fnmatch pattern can also be specified.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pattern`? | `null` \| `string` |

#### 반환 형식:

`string`[]

***

### tagForeach()

> **tagForeach**(`callback`): `void`

iterate over all tags calling `callback` on each.
the callback is provided the tag id and name

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `callback` | (`oid`, `name`) => `boolean` |

#### 반환 형식:

`void`

***

### deleteTag()

> **deleteTag**(`name`): `void`

Delete an existing tag reference.

The tag name will be checked for validity, see `isValidTagName` for some rules
about valid names.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`void`

***

### createTag()

> **createTag**(`name`, `target`, `message`, `options`?): `string`

Create a new tag in the repository from an object

A new reference will also be created pointing to this tag object. If
`force` is true and a reference already exists with the given name,
it'll be replaced.

The message will not be cleaned up.

The tag name will be checked for validity. You must avoid the characters
'~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
{" which have special meaning to revparse.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |
| `target` | [`GitObject`](GitObject.md) |
| `message` | `string` |
| `options`? | `null` \| [`CreateTagOptions`](../interfaces/CreateTagOptions.md) |

#### 반환 형식:

`string`

***

### createAnnotationTag()

> **createAnnotationTag**(`name`, `target`, `message`, `options`?): `string`

Create a new tag in the repository from an object without creating a reference.

The message will not be cleaned up.

The tag name will be checked for validity. You must avoid the characters
'~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
{" which have special meaning to revparse.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |
| `target` | [`GitObject`](GitObject.md) |
| `message` | `string` |
| `options`? | `null` \| [`CreateAnnotationTagOptions`](../interfaces/CreateAnnotationTagOptions.md) |

#### 반환 형식:

`string`

***

### createLightweightTag()

> **createLightweightTag**(`name`, `target`, `options`?): `string`

Create a new lightweight tag pointing at a target object

A new direct reference will be created pointing to this target object.
If force is true and a reference already exists with the given name,
it'll be replaced.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |
| `target` | [`GitObject`](GitObject.md) |
| `options`? | `null` \| [`CreateLightweightTagOptions`](../interfaces/CreateLightweightTagOptions.md) |

#### 반환 형식:

`string`

***

### getTree()

> **getTree**(`oid`): [`Tree`](Tree.md)

Lookup a reference to one of the objects in a repository.

Throws error if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Tree`](Tree.md)

***

### findTree()

> **findTree**(`oid`): `null` \| [`Tree`](Tree.md)

Lookup a reference to one of the objects in a repository.

Returns `null` if it does not exist

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Tree`](Tree.md)
