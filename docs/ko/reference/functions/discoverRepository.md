[es-git](../globals.md) / discoverRepository

# 함수: discoverRepository()

> **discoverRepository**(`path`, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

Attempt to open an already-existing repository at or above `path`.

This starts at `path` and looks up the filesystem hierarchy
until it finds a repository.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |
| `signal`? | `null` \| `AbortSignal` |

## 반환 형식:

`Promise`\<[`Repository`](../classes/Repository.md)\>
