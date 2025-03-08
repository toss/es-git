[es-git](../globals.md) / RepositoryCloneOptions

# 인터페이스: RepositoryCloneOptions

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="bare"></a> `bare?` | `boolean` | Indicate whether the repository will be cloned as a bare repository or not. |
| <a id="branch"></a> `branch?` | `string` | Specify the name of the branch to check out after the clone. If not specified, the remote's default branch will be used. |
| <a id="recursive"></a> `recursive?` | `boolean` | Clone a remote repository, initialize and update its submodules recursively. This is similar to `git clone --recursive`. |
| <a id="fetch"></a> `fetch?` | [`FetchOptions`](FetchOptions.md) | Options which control the fetch. |
