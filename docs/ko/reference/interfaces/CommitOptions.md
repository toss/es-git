[es-git](../globals.md) / CommitOptions

# 인터페이스: CommitOptions

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="updateref"></a> `updateRef?` | `string` | - |
| <a id="author"></a> `author?` | [`SignaturePayload`](SignaturePayload.md) | Signature for author. If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur. |
| <a id="committer"></a> `committer?` | [`SignaturePayload`](SignaturePayload.md) | Signature for commiter. If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur. |
| <a id="parents"></a> `parents?` | `string`[] | - |
