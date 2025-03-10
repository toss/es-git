[es-git](../globals.md) / discoverRepository

# Function: discoverRepository()

> **discoverRepository**(`path`, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

Attempt to open an already-existing repository at or above `path`.

This starts at `path` and looks up the filesystem hierarchy
until it finds a repository.

## Parameters

| Parameter | Type |
| ------ | ------ |
| `path` | `string` |
| `signal`? | `null` \| `AbortSignal` |

## Returns

`Promise`\<[`Repository`](../classes/Repository.md)\>
