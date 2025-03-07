[es-git](../globals.md) / ProxyOptions

# Interface: ProxyOptions

Options which can be specified to various fetch operations.

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="auto"></a> `auto?` | `boolean` | Try to auto-detect the proxy from the git configuration. Note that this will override `url` specified before. |
| <a id="url"></a> `url?` | `string` | Specify the exact URL of the proxy to use. Note that this will override `auto` specified before. |
