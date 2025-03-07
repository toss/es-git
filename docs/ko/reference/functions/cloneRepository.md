[es-git](../globals.md) / cloneRepository

# 함수: cloneRepository()

> **cloneRepository**(`url`, `path`, `options`?, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

Clone a remote repository.

This will use the options configured so far to clone the specified URL
into the specified local path.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `url` | `string` |
| `path` | `string` |
| `options`? | `null` \| [`RepositoryCloneOptions`](../interfaces/RepositoryCloneOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

## 반환 형식:

`Promise`\<[`Repository`](../classes/Repository.md)\>
