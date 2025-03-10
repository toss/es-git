[es-git](../globals.md) / CommitOptions

# Interface: CommitOptions

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="updateref"></a> `updateRef?` | `string` | - |
| <a id="author"></a> `author?` | [`SignaturePayload`](SignaturePayload.md) | Signature for author. If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur. |
| <a id="committer"></a> `committer?` | [`SignaturePayload`](SignaturePayload.md) | Signature for commiter. If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur. |
| <a id="parents"></a> `parents?` | `string`[] | - |
