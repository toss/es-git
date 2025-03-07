[es-git](../globals.md) / Diff

# 클래스: Diff

The diff object that contains all individual file deltas.

This is an opaque structure which will be allocated by one of the diff
generator functions on the `Repository` structure (e.g. `diff_tree_to_tree`
or other `diff_*` functions).

## 메소드

### merge()

> **merge**(`diff`): `void`

Merge one diff into another.

This merges items from the "from" list into the "self" list.  The
resulting diff will have all items that appear in either list.
If an item appears in both lists, then it will be "merged" to appear
as if the old version was from the "onto" list and the new version
is from the "from" list (with the exception that if the item has a
pending DELETE in the middle, then it will show as deleted).

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `diff` | [`Diff`](Diff.md) |

#### 반환 형식:

`void`

***

### deltas()

> **deltas**(): [`Deltas`](Deltas.md)

Returns an iterator over the deltas in this diff.

#### 반환 형식:

[`Deltas`](Deltas.md)

***

### isSortedIcase()

> **isSortedIcase**(): `boolean`

Check if deltas are sorted case sensitively or insensitively.

#### 반환 형식:

`boolean`

***

### stats()

> **stats**(): [`DiffStats`](DiffStats.md)

Accumulate diff statistics for all patches.

#### 반환 형식:

[`DiffStats`](DiffStats.md)

***

### print()

> **print**(`options`?): `string`

Iterate over a diff generating formatted text output.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `options`? | `null` \| [`DiffPrintOptions`](../interfaces/DiffPrintOptions.md) |

#### 반환 형식:

`string`
