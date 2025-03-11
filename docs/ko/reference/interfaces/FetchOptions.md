[es-git](../globals.md) / FetchOptions

# 인터페이스: FetchOptions

다양한 `fetch` 작업에서 사용할 수 있는 옵션이에요.

## 속성

| 속성                                              | 유형                                                    | 설명                                                                                         |
|-------------------------------------------------|-------------------------------------------------------|--------------------------------------------------------------------------------------------|
| <a id="credential"></a> `credential?`           | [`Credential`](../type-aliases/Credential.md)         | 인증 정보를 설정해요.                                                                               |
| <a id="proxy"></a> `proxy?`                     | [`ProxyOptions`](ProxyOptions.md)                     | `fetch` 작업에서 사용할 프록시(proxy) 옵션을 설정해요.                                                      |
| <a id="prune"></a> `prune?`                     | [`FetchPrune`](../type-aliases/FetchPrune.md)         | `fetch` 후에 불필요한 리모트 브랜치를 삭제할지 여부를 설정해요.                                                     |
| <a id="depth"></a> `depth?`                     | `number`                                              | `fetch` 깊이를 설정해요. 0 이하의 값을 설정하면 제한 없이 모든 데이터를 가져와요(제한이 없는 것과 동일).                          |
| <a id="downloadtags"></a> `downloadTags?`       | [`AutotagOption`](../type-aliases/AutotagOption.md)   | 리모트 리포지토리의 태그(tag)를 어떻게 가져올지 설정해요. 기본값은 자동으로 태그를 따라가도록 설정돼 있어요.                             |
| <a id="followredirects"></a> `followRedirects?` | [`RemoteRedirect`](../type-aliases/RemoteRedirect.md) | 리모트 리포지토리의 리디렉션을 허용할지 설정해요. 기본적으로 Git은 초기 요청(`/info/refs`)에서는 리디렉션을 따르지만, 이후 요청에서는 따르지 않아요. |
| <a id="customheaders"></a> `customHeaders?`     | `string`[]                                            | `fetch` 요청에 추가할 HTTP 헤더를 설정해요.                                                             |
