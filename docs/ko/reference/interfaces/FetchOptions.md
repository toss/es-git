[es-git](../globals.md) / FetchOptions

# 인터페이스: FetchOptions

Options which can be specified to various fetch operations.

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="credential"></a> `credential?` | [`Credential`](../type-aliases/Credential.md) | - |
| <a id="proxy"></a> `proxy?` | [`ProxyOptions`](ProxyOptions.md) | Set the proxy options to use for the fetch operation. |
| <a id="prune"></a> `prune?` | [`FetchPrune`](../type-aliases/FetchPrune.md) | Set whether to perform a prune after the fetch. |
| <a id="depth"></a> `depth?` | `number` | Set fetch depth, a value less or equal to 0 is interpreted as pull everything (effectively the same as not declaring a limit depth). |
| <a id="downloadtags"></a> `downloadTags?` | [`AutotagOption`](../type-aliases/AutotagOption.md) | Set how to behave regarding tags on the remote, such as auto-downloading tags for objects we're downloading or downloading all of them. The default is to auto-follow tags. |
| <a id="followredirects"></a> `followRedirects?` | [`RemoteRedirect`](../type-aliases/RemoteRedirect.md) | Set remote redirection settings; whether redirects to another host are permitted. By default, git will follow a redirect on the initial request (`/info/refs`), but not subsequent requests. |
| <a id="customheaders"></a> `customHeaders?` | `string`[] | Set extra headers for this fetch operation. |
