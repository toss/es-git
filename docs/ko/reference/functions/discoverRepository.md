[es-git](../globals.md) / discoverRepository

# 함수: discoverRepository()

> **discoverRepository**(`path`, `signal`?): `Promise`\<[`Repository`](../classes/Repository.md)\>

지정한 경로 또는 그 상위 디렉터리에서 기존 Git 리포지토리를 찾아 열어요.

이 함수는 `path`에서 시작해 파일 시스템을 위로 탐색하면서 리포지토리를 찾을 때까지 반복해요.

## 매개변수

| 매개변수      | 유형                      |
|-----------|-------------------------|
| `path`    | `string`                |
| `signal`? | `null` \| `AbortSignal` |

## 반환 형식:

`Promise`\<[`Repository`](../classes/Repository.md)\>
