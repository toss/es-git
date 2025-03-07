[es-git](../globals.md) / openRepository

# Function: openRepository()

> **openRepository**(`path`, `options`?, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

Attempt to open an already-existing repository at `path`.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |
| `options`? | `null` \| [`RepositoryOpenOptions`](../interfaces/RepositoryOpenOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

## Returns

`Promise`\<[`Repository`](../classes/Repository.md)\>
