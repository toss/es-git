[es-git](../globals.md) / Revwalk

# Class: Revwalk

A revwalk allows traversal of the commit graph defined by including one or
more leaves and excluding one or more roots.

## Methods

### reset()

> **reset**(): `this`

Reset a revwalk to allow re-configuring it.

The revwalk is automatically reset when iteration of its commits
completes.

#### Returns

`this`

***

### setSorting()

> **setSorting**(`sort`): `this`

Set the order in which commits are visited.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `sort` | `number` |

#### Returns

`this`

***

### simplifyFirstParent()

> **simplifyFirstParent**(): `this`

Simplify the history by first-parent

No parents other than the first for each commit will be enqueued.

#### Returns

`this`

***

### push()

> **push**(`oid`): `this`

Mark a commit to start traversal from.

The given OID must belong to a commitish on the walked repository.

The given commit will be used as one of the roots when starting the
revision walk. At least one commit must be pushed onto the walker before
a walk can be started.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `oid` | `string` |

#### Returns

`this`

***

### pushHead()

> **pushHead**(): `this`

Push the repository's HEAD

For more information, see `push`.

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `glob` | `string` |

#### Returns

`this`

***

### pushRange()

> **pushRange**(`range`): `this`

Push and hide the respective endpoints of the given range.

The range should be of the form `<commit>..<commit>` where each
`<commit>` is in the form accepted by `revparse_single`. The left-hand
commit will be hidden and the right-hand commit pushed.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `range` | `string` |

#### Returns

`this`

***

### pushRef()

> **pushRef**(`reference`): `this`

Push the OID pointed to by a reference

The reference must point to a commitish.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `reference` | `string` |

#### Returns

`this`

***

### hide()

> **hide**(`oid`): `this`

Mark a commit as not of interest to this revwalk.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `oid` | `string` |

#### Returns

`this`

***

### hideHead()

> **hideHead**(): `this`

Hide the repository's HEAD

For more information, see `hide`.

#### Returns

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

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `glob` | `string` |

#### Returns

`this`

***

### hideRef()

> **hideRef**(`reference`): `this`

Hide the OID pointed to by a reference.

The reference must point to a commitish.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `reference` | `string` |

#### Returns

`this`

***

### \[iterator\]()

> **\[iterator\]**(): `Iterator`\<`string`, `void`, `void`\>

#### Returns

`Iterator`\<`string`, `void`, `void`\>
