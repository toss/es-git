[es-git](../globals.md) / cloneRepository

# 함수: cloneRepository()

> **cloneRepository**(`url`, `path`, `options`?, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

리모트 리포지토리를 클론(clone)해요.

지정한 URL의 리모트 리포지토리를 로컬 경로에 복제하며, 제공된 옵션을 사용해 설정할 수 있어요.

## 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `url` | `string` |
| `path` | `string` |
| `options`? | `null` \| [`RepositoryCloneOptions`](../interfaces/RepositoryCloneOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

## 반환 형식:

`Promise`\<[`Repository`](../classes/Repository.md)\>
