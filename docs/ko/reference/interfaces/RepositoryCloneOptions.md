[es-git](../globals.md) / RepositoryCloneOptions

# 인터페이스: RepositoryCloneOptions

## 속성

| 속성                                  | 유형                                | 설명                                                                           |
|-------------------------------------|-----------------------------------|------------------------------------------------------------------------------|
| <a id="bare"></a> `bare?`           | `boolean`                         | 리포지토리를 bare 리포지토리로 클론할지 여부를 나타내요.                                            |
| <a id="branch"></a> `branch?`       | `string`                          | 클론 이후 체크아웃할 브랜치의 이름을 지정해요. 지정하지 않을 경우 리모트의 기본 브랜치가 사용돼요.                     |
| <a id="recursive"></a> `recursive?` | `boolean`                         | 리모트 리포지토리를 클론하면서 서브모듈을 재귀적으로 초기화하고 업데이트해요. 이는 `git clone --recursive`와 유사해요. |
| <a id="fetch"></a> `fetch?`         | [`FetchOptions`](FetchOptions.md) | fetch를 제어하는 옵션들을 설정해요.                                                       |
