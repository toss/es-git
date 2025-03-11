[es-git](../globals.md) / PushOptions

# 인터페이스: PushOptions

Git 푸시(push) 동작을 제어하기 위한 옵션이에요.

## 속성

| 속성                                              | 유형                                                    | 설명                                                                                                                                                      |
|-------------------------------------------------|-------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| <a id="credential"></a> `credential?`           | [`Credential`](../type-aliases/Credential.md)         | -                                                                                                                                                       |
| <a id="proxy"></a> `proxy?`                     | [`ProxyOptions`](ProxyOptions.md)                     | 푸시 작업에 사용할 프록시 옵션을 설정해요.                                                                                                                                |
| <a id="pbparallelism"></a> `pbParallelism?`     | `number`                                              | 원격으로 푸시할 때, pack 파일 생성을 요구하는 전송 방식을 사용하는 경우, packbuilder가 해당 pack 파일을 생성할 때 사용할 워커 스레드의 수를 제어해요. 0으로 설정하면 packbuilder가 자동으로 생성할 스레드 수를 결정하며, 기본값은 0이에요. |
| <a id="followredirects"></a> `followRedirects?` | [`RemoteRedirect`](../type-aliases/RemoteRedirect.md) | 원격 리디렉션 설정을 지정해요. 다른 호스트로의 리디렉션 허용 여부를 결정할 수 있어요. 기본적으로 git은 최초 요청(`/info/refs`)에서의 리디렉션은 따르지만, 이후 요청에서는 리디렉션을 따르지 않아요.                                 |
| <a id="customheaders"></a> `customHeaders?`     | `string`[]                                            | 이 푸시 작업에 추가 헤더를 설정해요.                                                                                                                                   |
| <a id="remoteoptions"></a> `remoteOptions?`     | `string`[]                                            | 리모트에 전달할 "푸시 옵션"을 설정해요.                                                                                                                                 |
