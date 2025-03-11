[es-git](../globals.md) / Revwalk

# 클래스: Revwalk

Revwalk는 하나 이상의 리프를 포함하고 하나 이상의 루트를 제외하여 정의된 커밋 그래프를 순회할 수 있도록 해줘요.

## 메소드

### reset()

> **reset**(): `this`

Revwalk를 재구성할 수 있도록 초기화해요.

커밋 순회가 완료되면 Revwalk는 자동으로 초기화돼요.

#### 반환 형식:

`this`

***

### setSorting()

> **setSorting**(`sort`): `this`

커밋을 방문하는 순서를 설정해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `sort` | `number` |

#### 반환 형식:

`this`

***

### simplifyFirstParent()

> **simplifyFirstParent**(): `this`

첫 번째 부모를 기준으로 히스토리를 단순화해요.

각 커밋에 대해 첫 번째 부모 이외의 부모들은 큐에 추가되지 않아요.

#### 반환 형식:

`this`

***

### push()

> **push**(`oid`): `this`

순회의 시작점으로 사용할 커밋을 지정해요.

지정된 OID는 순회하는 리포지토리의 커밋에 속해야 해요.

순회 시작 시 이 커밋은 루트 중 하나로 사용돼요. 순회를 시작하기 전에 최소 한 개의 커밋이 푸시되어야 해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`this`

***

### pushHead()

> **pushHead**(): `this`

리포지토리의 HEAD를 푸시해요.

자세한 내용은 `push`를 참고하세요.

#### 반환 형식:

`this`

***

### pushGlob()

> **pushGlob**(`glob`): `this`

일치하는 레퍼런스들을 푸시해요.

주어진 glob 패턴과 일치하는 레퍼런스가 가리키는 OID들이 Revwalk에 푸시돼요.

glob에 '?', '*' 또는 '['가 없으면 선행하는 'refs/'와 후행하는 `/\*`가 암시돼요.

해당 glob과 일치하지만 커밋을 가리키지 않는 레퍼런스는 무시돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `glob` | `string` |

#### 반환 형식:

`this`

***

### pushRange()

> **pushRange**(`range`): `this`

주어진 범위의 끝점을 각각 푸시 및 숨겨요.

범위는 `<commit>..<commit>` 형식이어야 하며, 각각의 `<commit>`은 `revparseSingle`에서 허용하는 형식이에요. 왼쪽 커밋은 숨겨지고 오른쪽 커밋이 푸시돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `range` | `string` |

#### 반환 형식:

`this`

***

### pushRef()

> **pushRef**(`reference`): `this`

레퍼런스가 가리키는 OID를 푸시해요.

해당 레퍼런스는 커밋을 가리켜야 해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `reference` | `string` |

#### 반환 형식:

`this`

***

### hide()

> **hide**(`oid`): `this`

Revwalk에서 해당 커밋을 관심 대상에서 제외해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`this`

***

### hideHead()

> **hideHead**(): `this`

리포지토리의 HEAD를 숨겨요.

자세한 내용은 `hide`를 참고하세요.

#### 반환 형식:

`this`

***

### hideGlob()

> **hideGlob**(`glob`): `this`

일치하는 레퍼런스들을 숨겨요.

주어진 glob 패턴과 일치하는 레퍼런스 및 이들이 가리키는 OID와 해당 조상들은 Revwalk 출력에서 숨겨져요.

glob에 '?', '*' 또는 '['가 없으면 선행하는 'refs/'와 후행하는 `/\*`가 암시돼요.

해당 glob과 일치하지만 커밋을 가리키지 않는 레퍼런스는 무시돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `glob` | `string` |

#### 반환 형식:

`this`

***

### hideRef()

> **hideRef**(`reference`): `this`

레퍼런스가 가리키는 OID를 숨겨요.

해당 레퍼런스는 커밋을 가리켜야 해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `reference` | `string` |

#### 반환 형식:

`this`

***

### \[iterator\]()

> **\[iterator\]**(): `Iterator`\<`string`, `void`, `void`\>

#### 반환 형식:

`Iterator`\<`string`, `void`, `void`\>
