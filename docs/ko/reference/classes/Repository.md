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

리포지토리에 새 커밋을 만들어요.

`updateRef`가 `null`이 아니면, 해당 참조를 새로운 커밋을 가리키도록 업데이트해요.  
참조가 직접적인 것이 아니라면, 직접 참조로 변환돼요.  
`"HEAD"`를 사용하면 현재 브랜치의 HEAD를 이 커밋을 가리키도록 업데이트해요.  
참조가 존재하지 않으면 생성하고, 존재하면 첫 번째 부모는 해당 브랜치의 마지막 커밋이어야 해요.

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

두 개의 트리 개체 간 차이를 비교하는 diff를 생성해요.

이는 `git diff <old-tree> <new-tree>`와 동일해요.  
첫 번째 트리는 "oldFile" 역할을 하고, 두 번째 트리는 "newFile" 역할을 해요.  
`null`을 전달하면 빈 트리로 간주하지만, `oldTree`와 `newTree` 모두 `null`이면 오류가 발생해요.

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

두 개의 인덱스 개체 간 차이를 비교하는 diff를 생성해요.

첫 번째 인덱스는 "oldFile", 두 번째 인덱스는 "newFile" 역할을 해요.

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

리포지토리의 인덱스와 작업 디렉터리(workdir) 간 차이를 비교하는 diff를 생성해요.

이 동작은 `git diff` 명령과 동일해요.  
`git diff`와 `git diff HEAD`의 차이점 및 libgit2에서 `git diff <treeish>`를 구현하는 방법은 아래 `diffTreeToWorkdir` 설명을 참고하세요.

인덱스는 "oldFile" 역할을 하고, 작업 디렉터리는 "newFile" 역할을 해요.

`index`에 `null`을 전달하면 리포지토리의 기존 인덱스를 사용해요.  
이 경우, 인덱스가 변경되었으면 디스크에서 다시 불러온 후 diff를 생성해요.

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

트리와 작업 디렉터리 간 차이를 비교하는 diff를 생성해요.

`oldTree`는 "oldFile" 역할을 하고, 작업 디렉터리는 "newFile" 역할을 해요.

이 메서드는 `git diff <treeish>` 또는 `git diff-index <treeish>`와 동일하지 않아요.  
해당 명령어들은 인덱스 정보를 사용하지만, 이 메서드는 인덱스 상태와 관계없이 트리와 실제 파일 간 차이만 반환해요.  
`git diff <treeish>`와 동일한 결과를 얻으려면 `diffTreeToWorkdirWithIndex`를 사용하세요.

예를 들어, 파일을 삭제한 뒤 다시 작업 디렉터리에 복원하고 수정한 경우를 생각해 볼 수 있어요.  
이 메서드는 해당 파일을 '수정됨(modified)' 상태로 표시하지만, `git diff`는 '삭제됨(deleted)' 상태로 표시할 거예요.  
그 이유는 `git diff`가 인덱스의 삭제 상태를 반영하기 때문이에요.

`oldTree`에 `null`을 전달하면 빈 트리로 간주해요.

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

트리와 작업 디렉터리 간 차이를 비교할 때, 인덱스 정보를 포함해서 diff를 생성해요.

이 메서드는 `git diff <tree>` 명령어와 동일한 동작을 해요.  
즉, 트리와 인덱스를 비교한 후, 인덱스와 작업 디렉터리를 비교한 결과를 하나의 diff로 합쳐서  
스테이징된 삭제 파일 등도 포함한 최종 diff를 반환해요.

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

사용자 지정 인덱스가 설정되지 않았다면, 리포지토리의 기본 인덱스(`.git/index`)를 반환해요.

#### 반환 형식:

[`Index`](Index.md)

***

### findObject()

> **findObject**(`oid`): `null` \| [`GitObject`](GitObject.md)

리포지토리에서 특정 개체를 찾아요.

개체가 존재하지 않으면 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`GitObject`](GitObject.md)

***

### getObject()

> **getObject**(`oid`): [`GitObject`](GitObject.md)

리포지토리에서 특정 개체를 찾아요.

개체가 존재하지 않으면 오류가 발생해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### findReference()

> **findReference**(`name`): `null` \| [`Reference`](Reference.md)

리포지토리에 특정 레퍼런스를 찾아요.

레퍼런스가 존재하지 않으면 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`null` \| [`Reference`](Reference.md)

***

### getReference()

> **getReference**(`name`): [`Reference`](Reference.md)

리포지토리에 특정 레퍼런스를 찾아요.

레퍼런스가 존재하지 않으면 오류가 발생해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

[`Reference`](Reference.md)

***

### remoteNames()

> **remoteNames**(): `string`[]

리포지토리에 등록된 모든 리모트 목록을 가져와요.

#### 반환 형식:

`string`[]

***

### getRemote()

> **getRemote**(`name`): [`Remote`](Remote.md)

리포지토리에서 특정 리모트를 가져와요.

리모트가 존재하지 않으면 오류가 발생해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

[`Remote`](Remote.md)

***

### findRemote()

> **findRemote**(`name`): `null` \| [`Remote`](Remote.md)

리포지토리에서 특정 리모트를 가져와요.

리모트가 존재하지 않으면 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`null` \| [`Remote`](Remote.md)

***

### createRemote()

> **createRemote**(`name`, `url`, `options`?): [`Remote`](Remote.md)

리포지토리 기본 설정된 fetch, refspec으로 리모트를 새로 생성해요.

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

이 리포지토리가 `bare`(작업 디렉터리가 없는 리포지토리)인지 확인해요.

#### 반환 형식:

`boolean`

***

### isShallow()

> **isShallow**(): `boolean`

이 리포지토리가 `shallow`(히스토리가 일부만 있는 얕은 복제본)인지 확인해요.

#### 반환 형식:

`boolean`

***

### isWorktree()

> **isWorktree**(): `boolean`

이 리포지토리가 `worktree`인지 확인해요.

#### 반환 형식:

`boolean`

***

### isEmpty()

> **isEmpty**(): `boolean`

리포지토리가 비어 있는지 확인해요.

#### 반환 형식:

`boolean`

***

### path()

> **path**(): `string`

일반 리포지토리의 경우 `.git` 디렉터리의 경로를 반환하고,  
`bare` 리포지토리의 경우 자체의 경로를 반환해요.

#### 반환 형식:

`string`

***

### state()

> **state**(): [`RepositoryState`](../type-aliases/RepositoryState.md)

현재 리포지토리의 상태를 반환해요.

#### 반환 형식:

[`RepositoryState`](../type-aliases/RepositoryState.md)

***

### workdir()

> **workdir**(): `null` \| `string`

이 리포지토리의 작업 디렉터리 경로를 가져와요.

이 리포지토리가 `bare`이면 `null`을 반환해요.

#### 반환 형식:

`null` \| `string`

***

### head()

> **head**(): [`Reference`](Reference.md)

현재 `HEAD`가 가리키는 레퍼런스를 가져와요.

#### 반환 형식:

[`Reference`](Reference.md)

***

### setHead()

> **setHead**(`refname`): `void`

리포지토리의 `HEAD`를 지정된 레퍼런스로 변경해요.

- 지정된 레퍼런스가 트리나 블롭을 가리키면 `HEAD`는 변경되지 않고 오류가 발생해요.
- 지정된 레퍼런스가 브랜치를 가리키면 `HEAD`는 해당 브랜치를 가리키도록 설정돼요.
  - 이미 브랜치에 연결된 상태라면 그대로 유지되고, 연결되지 않은 상태라면 연결돼요.
  - 브랜치가 아직 존재하지 않으면 오류 없이 `HEAD`가 해당 브랜치에 연결된 상태로 설정돼요.
- 지정된 레퍼런스가 커밋을 가리키면 `HEAD`가 분리된(detached) 상태가 되어 해당 커밋을 직접 가리켜요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refname` | `string` |

#### 반환 형식:

`void`

***

### revparse()

> **revparse**(`spec`): [`Revspec`](../interfaces/Revspec.md)

주어진 `spec`에 대해 `rev-parse` 연산을 수행해요.

연산 결과로 리비전 명세가 반환되거나, 오류가 발생할 수 있어요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `spec` | `string` |

#### 반환 형식:

[`Revspec`](../interfaces/Revspec.md)

***

### revparseSingle()

> **revparseSingle**(`spec`): `string`

주어진 리비전 문자열을 사용해 하나의 개체를 찾아요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `spec` | `string` |

#### 반환 형식:

`string`

***

### revwalk()

> **revwalk**(): [`Revwalk`](Revwalk.md)

커밋 그래프를 순회할 수 있는 `Revwalk`를 생성해요.

#### 반환 형식:

[`Revwalk`](Revwalk.md)

***

### findTag()

> **findTag**(`oid`): `null` \| [`Tag`](Tag.md)

리포지토리에서 특정 태그를 찾아요.

태그가 존재하지 않으면 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Tag`](Tag.md)

***

### getTag()

> **getTag**(`oid`): [`Tag`](Tag.md)

리포지토리에서 특정 태그를 찾아요.

태그가 존재하지 않으면 오류를 발생시켜요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Tag`](Tag.md)

***

### tagNames()

> **tagNames**(`pattern`?): `string`[]

리포지토리에 있는 모든 태그 목록을 가져와요.

선택적으로 `fnmatch` 패턴을 지정할 수도 있어요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pattern`? | `null` \| `string` |

#### 반환 형식:

`string`[]

***

### tagForeach()

> **tagForeach**(`callback`): `void`

리포지토리의 모든 태그를 순회하면서 `callback` 함수를 호출해요.  
콜백 함수에는 태그 ID와 이름이 전달돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `callback` | (`oid`, `name`) => `boolean` |

#### 반환 형식:

`void`

***

### deleteTag()

> **deleteTag**(`name`): `void`

기존 태그 참조를 삭제해요.

태그 이름의 유효성을 검사해요. 유효한 태그 이름 규칙은 `isValidTagName`을 참고하세요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `name` | `string` |

#### 반환 형식:

`void`

***

### createTag()

> **createTag**(`name`, `target`, `message`, `options`?): `string`

리포지토리에서 지정된 개체를 기반으로 새 태그를 생성해요.

새로운 레퍼런스도 함께 생성돼요. `force`가 `true`이면 같은 이름의 참조가 이미 존재해도 덮어써요.

태그 이름의 유효성을 검사해요.  
`~`, `^`, `:`, `\`, `?`, `[`, `*` 같은 문자와 `..`, `@{` 등의 문자열은 사용하면 안 돼요.

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

레퍼런스를 생성하지 않고, 지정된 개체를 기반으로 새 태그를 만들어요.

태그 이름의 유효성을 검사해요.  
`~`, `^`, `:`, `\`, `?`, `[`, `*` 같은 문자와 `..`, `@{` 등의 문자열은 사용하면 안 돼요.

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

대상 개체를 가리키는 새로운 경량 태그(lightweight tag)를 생성해요.

새로운 직접 참조(direct reference)가 생성되며, `force`가 `true`이면 같은 이름의 레퍼런스가 존재하더라도 덮어써요.

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

리포지토리 내의 개체에 대한 참조를 조회해요.

트리가 존재하지 않으면 오류를 발생시켜요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

[`Tree`](Tree.md)

***

### findTree()

> **findTree**(`oid`): `null` \| [`Tree`](Tree.md)

리포지토리 내의 개체에 대한 참조를 조회해요.

트리가 존재하지 않으면 `null`을 반환해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`null` \| [`Tree`](Tree.md)
