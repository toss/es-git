[es-git](../globals.md) / Tree

# 클래스: Tree

Git [트리(Tree)][1]를 나타내는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### id()

> **id**(): `string`

리포지토리 개체의 id (SHA1)를 가져와요.

#### 반환 형식:

`string`

***

### len()

> **len**(): `bigint`

이 트리에 나열된 항목의 개수를 가져와요.

#### 반환 형식:

`bigint`

***

### isEmpty()

> **isEmpty**(): `boolean`

항목이 없으면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### iter()

> **iter**(): [`TreeIter`](TreeIter.md)

이 트리의 항목들을 순회할 수 있는 이터레이터를 반환해요.

#### 반환 형식:

[`TreeIter`](TreeIter.md)

***

### walk()

> **walk**(`mode`, `callback`): `void`

후위 또는 전위 순회(post 또는 pre-order) 방식으로 트리와 그 하위 트리의 항목들을 탐색해요. 순회하는 트리의 각 노드마다 콜백 함수가 실행돼요. 이 함수의 반환 코드가 순회의 진행 방식을 결정해요.

libgit2에서는 반환값이 정수여야 하며, 0은 정상 방문, 1은 해당 노드를 건너뛰고, -1은 순회를 완전히 중단함을 의미해요. 자세한 내용은 [libgit2 문서][1]를 참고하세요.

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

SHA 값으로 트리 항목을 조회해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `id` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### get()

> **get**(`index`): `null` \| [`TreeEntry`](TreeEntry.md)

트리 내 위치로 트리 항목을 조회해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `index` | `number` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getName()

> **getName**(`filename`): `null` \| [`TreeEntry`](TreeEntry.md)

파일 이름으로 트리 항목을 조회해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `filename` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### getPath()

> **getPath**(`path`): `null` \| [`TreeEntry`](TreeEntry.md)

상대 경로를 이용해 트리 또는 그 하위 트리에 포함된 트리 항목을 가져와요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |

#### 반환 형식:

`null` \| [`TreeEntry`](TreeEntry.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

이 트리를 `GitObject`로 사용할 수 있도록 형변환해요.

#### 반환 형식:

[`GitObject`](GitObject.md)
