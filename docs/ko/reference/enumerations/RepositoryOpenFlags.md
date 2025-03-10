[es-git](../globals.md) / RepositoryOpenFlags

# 열거형: RepositoryOpenFlags

리포지토리를 여는데 사용되는 플래그

## 포함된 값

| 포함된 값                            | 값    | 설명                                                                |
|----------------------------------|------|-------------------------------------------------------------------|
| <a id="nosearch"></a> `NoSearch` | `1`  | 상위 디렉터리를 찾지 않고 지정된 경로만 탐색                                         |
| <a id="crossfs"></a> `CrossFS`   | `2`  | 상위 디렉터리                                                           |
| <a id="bare"></a> `Bare`         | `4`  | Force opening as a bare repository, and defer loading its config. |
| <a id="nodotgit"></a> `NoDotGit` | `8`  | 지정된 리포지토리 경로에 `/.git`을 추가하지 않음                                    |
| <a id="fromenv"></a> `FromEnv`   | `16` | `$GIT_DIR`와 같은 환경 변수를 존중                                          |
