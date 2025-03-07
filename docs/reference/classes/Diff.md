[es-git](../globals.md) / Diff

# Class: Diff

The diff object that contains all individual file deltas.

This is an opaque structure which will be allocated by one of the diff
generator functions on the `Repository` structure (e.g. `diff_tree_to_tree`
or other `diff_*` functions).

## Methods

### merge()

> **merge**(`diff`): `void`

Merge one diff into another.

This merges items from the "from" list into the "self" list.  The
resulting diff will have all items that appear in either list.
If an item appears in both lists, then it will be "merged" to appear
as if the old version was from the "onto" list and the new version
is from the "from" list (with the exception that if the item has a
pending DELETE in the middle, then it will show as deleted).

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `diff` | [`Diff`](Diff.md) |

#### Returns

`void`

***

### deltas()

> **deltas**(): [`Deltas`](Deltas.md)

Returns an iterator over the deltas in this diff.

#### Returns

[`Deltas`](Deltas.md)

***

### isSortedIcase()

> **isSortedIcase**(): `boolean`

Check if deltas are sorted case sensitively or insensitively.

#### Returns

`boolean`

***

### stats()

> **stats**(): [`DiffStats`](DiffStats.md)

Accumulate diff statistics for all patches.

#### Returns

[`DiffStats`](DiffStats.md)

***

### print()

> **print**(`options`?): `string`

Iterate over a diff generating formatted text output.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `options`? | `null` \| [`DiffPrintOptions`](../interfaces/DiffPrintOptions.md) |

#### Returns

`string`
