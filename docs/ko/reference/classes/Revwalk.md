[es-git](../globals.md) / Revwalk

# 클래스: Revwalk

A revwalk allows traversal of the commit graph defined by including one or
more leaves and excluding one or more roots.

## 메소드

### reset()

> **reset**(): `this`

Reset a revwalk to allow re-configuring it.

The revwalk is automatically reset when iteration of its commits
completes.

#### 반환 형식:

`this`

***

### setSorting()

> **setSorting**(`sort`): `this`

Set the order in which commits are visited.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `sort` | `number` |

#### 반환 형식:

`this`

***

### simplifyFirstParent()

> **simplifyFirstParent**(): `this`

Simplify the history by first-parent

No parents other than the first for each commit will be enqueued.

#### 반환 형식:

`this`

***

### push()

> **push**(`oid`): `this`

Mark a commit to start traversal from.

The given OID must belong to a commitish on the walked repository.

The given commit will be used as one of the roots when starting the
revision walk. At least one commit must be pushed onto the walker before
a walk can be started.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`this`

***

### pushHead()

> **pushHead**(): `this`

Push the repository's HEAD

For more information, see `push`.

#### 반환 형식:

`this`

***

### pushGlob()

> **pushGlob**(`glob`): `this`

Push matching references

The OIDs pointed to by the references that match the given glob pattern
will be pushed to the revision walker.

A leading 'refs/' is implied if not present as well as a trailing `/ \
*` if the glob lacks '?', ' \ *' or '['.

Any references matching this glob which do not point to a commitish
will be ignored.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `glob` | `string` |

#### 반환 형식:

`this`

***

### pushRange()

> **pushRange**(`range`): `this`

Push and hide the respective endpoints of the given range.

The range should be of the form `<commit>..<commit>` where each
`<commit>` is in the form accepted by `revparseSingle`. The left-hand
commit will be hidden and the right-hand commit pushed.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `range` | `string` |

#### 반환 형식:

`this`

***

### pushRef()

> **pushRef**(`reference`): `this`

Push the OID pointed to by a reference

The reference must point to a commitish.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `reference` | `string` |

#### 반환 형식:

`this`

***

### hide()

> **hide**(`oid`): `this`

Mark a commit as not of interest to this revwalk.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `oid` | `string` |

#### 반환 형식:

`this`

***

### hideHead()

> **hideHead**(): `this`

Hide the repository's HEAD

For more information, see `hide`.

#### 반환 형식:

`this`

***

### hideGlob()

> **hideGlob**(`glob`): `this`

Hide matching references.

The OIDs pointed to by the references that match the given glob pattern
and their ancestors will be hidden from the output on the revision walk.

A leading 'refs/' is implied if not present as well as a trailing `/ \
*` if the glob lacks '?', ' \ *' or '['.

Any references matching this glob which do not point to a commitish
will be ignored.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `glob` | `string` |

#### 반환 형식:

`this`

***

### hideRef()

> **hideRef**(`reference`): `this`

Hide the OID pointed to by a reference.

The reference must point to a commitish.

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
