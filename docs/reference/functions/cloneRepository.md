[es-git](../globals.md) / cloneRepository

# Function: cloneRepository()

> **cloneRepository**(`url`, `path`, `options`?, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

Clone a remote repository.

This will use the options configured so far to clone the specified URL
into the specified local path.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `url` | `string` |
| `path` | `string` |
| `options`? | `null` \| [`RepositoryCloneOptions`](../interfaces/RepositoryCloneOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

## Returns

`Promise`\<[`Repository`](../classes/Repository.md)\>
