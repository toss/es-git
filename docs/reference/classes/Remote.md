[es-git](../globals.md) / Remote

# Class: Remote

A structure representing a [remote][1] of a git repository.

[1]: http://git-scm.com/book/en/Git-Basics-Working-with-Remotes

## Methods

### name()

> **name**(): `null` \| `string`

Get the remote's name.

Returns `null` if this remote has not yet been named, and
Throws error if the URL is not valid utf-8

#### Returns

`null` \| `string`

***

### url()

> **url**(): `string`

Get the remote's URL.

Throws error if the URL is not valid utf-8

#### Returns

`string`

***

### pushurl()

> **pushurl**(): `null` \| `string`

Get the remote's URL.

Returns `null` if push url not exists, and
Throws error if the URL is not valid utf-8

#### Returns

`null` \| `string`

***

### refspecs()

> **refspecs**(): [`RefspecObject`](../interfaces/RefspecObject.md)[]

List all refspecs.

Filter refspec if has not valid src or dst with utf-8

#### Returns

[`RefspecObject`](../interfaces/RefspecObject.md)[]

***

### fetch()

> **fetch**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

Download new data and update tips

Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `refspecs` | `string`[] |
| `options`? | `null` \| [`FetchRemoteOptions`](../interfaces/FetchRemoteOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### Returns

`Promise`\<`void`\>

***

### push()

> **push**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

Perform a push

Perform all the steps for a push.
If no refspecs are passed, then the configured refspecs will be used.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `refspecs` | `string`[] |
| `options`? | `null` \| [`PushOptions`](../interfaces/PushOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### Returns

`Promise`\<`void`\>

***

### prune()

> **prune**(`options`?, `signal`?): `Promise`\<`void`\>

Prune tracking refs that are no longer present on remote

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `options`? | `null` \| [`PruneOptions`](../interfaces/PruneOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### Returns

`Promise`\<`void`\>

***

### defaultBranch()

> **defaultBranch**(`signal`?): `Promise`\<`string`\>

Get the remote’s default branch.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `signal`? | `null` \| `AbortSignal` |

#### Returns

`Promise`\<`string`\>
