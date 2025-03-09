[es-git](../globals.md) / Repository

# 클래스: Repository

Git 리포지토리를 나타내는 클래스로, 파일 시스템과 관련된 상태를 관리해요.

이 클래스는 libgit2에서 Git 리포지토리 사용과 동일해요.

## 메소드

### findCommit()

> **findCommit**(`oid`): `null` \| [`Commit`](Commit.md)

리포지토리에서 특정 커밋에 대한 레퍼런스를 조회해요.

해당 커밋이 존재하지 않는 경우 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Commit`](Commit.md)

***

### getCommit()

> **getCommit**(`oid`): [`Commit`](Commit.md)

리포지토리에서 특정 커밋에 대한 레퍼런스를 조회해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Commit`](Commit.md)

***

### commit()

> **commit**(`tree`, `message`, `options`?): `string`

리포지토리에 새 커밋을 생성해요.

`updateRef`가 `null`이 아닌 경우, 해당 이름의 레퍼런스가 이 커밋을 가리키도록 업데이트돼요.
레퍼런스가 직접적인(direct) 레퍼런스가 아닌 경우, 이를 직접 레퍼런스로 해석해요.
"HEAD"를 사용하여 현재 브랜치의 HEAD를 업데이트하고, 해당 HEAD가 이 커밋을 가리키도록 만들 수 있어요.
레퍼런스가 아직 존재하지 않는 경우 새로 생성돼요. 존재하는 경우, 첫 번째 부모는 반드시 이 브랜치의 현재 머리(tip)여야 해요.

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

두 트리 개체 간의 차이를 비교(diff)해요.

이 메서드는 `git diff <old-tree> <new-tree>` 명령과 동일해요.

첫 번째 트리는 delta의 "oldFile" 측에서 사용되며,
두 번째 트리는 delta의 "newFile" 측에서 사용돼요.
`null`을 전달하여 빈 트리를 나타낼 수 있지만,
`oldTree`와 `newTree` 둘 다에 대해 `null`을 전달하면 오류가 발생해요.

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

두 인덱스 객체 간의 차이를 비교(diff)해요.

첫 번째 인덱스는 delta의 "oldFile" 측에서 사용되며,
두 번째 인덱스는 delta의 "newFile" 측에서 사용돼요.

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

리포지토리 인덱스와 작업 디렉토리 간의 차이를 비교(diff)합니다.

이 작업은 `git diff` 명령과 일치해요.
`git diff`와 `git diff HEAD`의 차이점에 대한 설명 및
libgit2를 사용하여 `git diff <treeish>`를 에뮬레이트하는 방법에 대한 참고 사항은
`diffTreeToWorkdir` 설명을 확인하세요.

인덱스는 delta의 "oldFile" 측에서 사용되며,
작업 디렉토리는 delta의 "newFile" 측에서 사용돼요.

인덱스에 대해 `null`을 전달한 경우, 기존 리포지토리 인덱스가 사용돼요.
이 경우 diff가 생성되기 전에 인덱스가 디스크에서 새로 고쳐질 수 있어요(변경된 경우).

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

트리와 작업 디렉토리 간의 차이를 비교(diff)해요.

제공된 트리는 delta의 "oldFile" 측으로 사용되며,
작업 디렉토리는 delta의 "newFile" 측으로 사용돼요.

이 함수는 `git diff <treeish>` 또는 `git diff-index <treeish>` 명령과 동일하지 않아요.
이 명령은 인덱스 정보를 이용하지만, 이 함수는 트리와 작업 디렉토리 내 파일 간의 차이만을 반환해요.
인덱스 상태와는 상관없이 작동해요.
해당 명령을 사용하려면 `diffTreeToWorkdirWithIndex`를 사용하세요.

`diffTreeToWorkdirWithIndex`와 이 함수의 차이를 이해하려면, 파일이 스테이징된 상태에서 삭제된 후,
다시 작업 디렉토리에 추가되고 수정된 경우를 고려해볼 수 있어요.
`diffTreeToWorkdirWithIndex`의 결과는 해당 파일이 '수정(modified)'되었다고 나타내지만,
`git diff`는 '삭제(deleted)' 상태를 표시해요. 이는 삭제가 스테이징되었기 때문이에요.

또한, `tree`에 `null`을 전달하면 빈 트리가 사용돼요.

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

인덱스 데이터를 사용하여 트리와 작업 디렉토리 간의 차이를 비교(diff)해요.

스테이징된 삭제, 추적된 파일 등의 상태를 고려해요.
이 함수는 트리와 인덱스 간, 그리고 인덱스와 작업 디렉토리 간의 차이를 비교하여
스테이징된 삭제 등을 포함한 단일 diff로 결과를 합병하여 `git diff <tree>` 명령을 수행해요.

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

이 리포지토리의 인덱스 파일을 가져와요.

사용자 지정 인덱스가 설정되지 않은 경우,
리포지토리의 기본 인덱스(`.git/index`에 위치)가 반환돼요.

#### 반환 형식:

[`Index`](Index.md)

***

### findObject()

> **findObject**(`oid`): `null` \| [`GitObject`](GitObject.md)

리포지토리에서 개체를 조회해요.

해당 개체가 존재하지 않는 경우 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`GitObject`](GitObject.md)

***

### getObject()

> **getObject**(`oid`): [`GitObject`](GitObject.md)

리포지토리에서 개체를 가져와요.

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
