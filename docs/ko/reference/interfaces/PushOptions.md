[es-git](../globals.md) / PushOptions

# 인터페이스: PushOptions

Options to control the behavior of a git push.

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="credential"></a> `credential?` | [`Credential`](../type-aliases/Credential.md) | - |
| <a id="proxy"></a> `proxy?` | [`ProxyOptions`](ProxyOptions.md) | Set the proxy options to use for the push operation. |
| <a id="pbparallelism"></a> `pbParallelism?` | `number` | If the transport being used to push to the remote requires the creation of a pack file, this controls the number of worker threads used by the packbuilder when creating that pack file to be sent to the remote. If set to 0, the packbuilder will auto-detect the number of threads to create, and the default value is 1. |
| <a id="followredirects"></a> `followRedirects?` | [`RemoteRedirect`](../type-aliases/RemoteRedirect.md) | Set remote redirection settings; whether redirects to another host are permitted. By default, git will follow a redirect on the initial request (`/info/refs`), but not subsequent requests. |
| <a id="customheaders"></a> `customHeaders?` | `string`[] | Set extra headers for this push operation. |
| <a id="remoteoptions"></a> `remoteOptions?` | `string`[] | Set "push options" to deliver to the remote. |
