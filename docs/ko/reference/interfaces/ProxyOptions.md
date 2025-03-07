[es-git](../globals.md) / ProxyOptions

# 인터페이스: ProxyOptions

Options which can be specified to various fetch operations.

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="auto"></a> `auto?` | `boolean` | Try to auto-detect the proxy from the git configuration. Note that this will override `url` specified before. |
| <a id="url"></a> `url?` | `string` | Specify the exact URL of the proxy to use. Note that this will override `auto` specified before. |
