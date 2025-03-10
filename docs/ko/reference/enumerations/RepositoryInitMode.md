[es-git](../globals.md) / RepositoryInitMode

# 열거형: RepositoryInitMode

`RepositoryInitOptions`에 사용된 리포지토리 모드

## 포함된 값

| 포함된 값                                    | 값      | 설명                                                                            |
|------------------------------------------|--------|-------------------------------------------------------------------------------|
| <a id="sharedunmask"></a> `SharedUnmask` | `0`    | unmask로 구성된 권한 사용 (기본값)                                                       |
| <a id="sharedgroup"></a> `SharedGroup`   | `1533` | `--shared=group`을 사용하여 새 리포지토리를 그룹 쓰기 가능으로 chmod하고 stick 그룹 할당을 위해 "g+sx"를 사용 |
| <a id="sharedall"></a> `SharedAll`       | `1535` | `--shared=all`을 사용                                                            |
