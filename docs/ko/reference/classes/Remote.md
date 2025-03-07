[es-git](../globals.md) / Remote

# 클래스: Remote

A structure representing a [remote][1] of a git repository.

[1]: http://git-scm.com/book/en/Git-Basics-Working-with-Remotes

## 메소드

### name()

> **name**(): `null` \| `string`

Get the remote's name.

Returns `null` if this remote has not yet been named, and
Throws error if the URL is not valid utf-8

#### 반환 형식:

`null` \| `string`

***

### url()

> **url**(): `string`

Get the remote's URL.

Throws error if the URL is not valid utf-8

#### 반환 형식:

`string`

***

### pushurl()

> **pushurl**(): `null` \| `string`

Get the remote's URL.

Returns `null` if push url not exists, and
Throws error if the URL is not valid utf-8

#### 반환 형식:

`null` \| `string`

***

### refspecs()

> **refspecs**(): [`RefspecObject`](../interfaces/RefspecObject.md)[]

List all refspecs.

Filter refspec if has not valid src or dst with utf-8

#### 반환 형식:

[`RefspecObject`](../interfaces/RefspecObject.md)[]

***

### fetch()

> **fetch**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

Download new data and update tips

Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refspecs` | `string`[] |
| `options`? | `null` \| [`FetchRemoteOptions`](../interfaces/FetchRemoteOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`void`\>

***

### push()

> **push**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

Perform a push

Perform all the steps for a push.
If no refspecs are passed, then the configured refspecs will be used.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `refspecs` | `string`[] |
| `options`? | `null` \| [`PushOptions`](../interfaces/PushOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`void`\>

***

### prune()

> **prune**(`options`?, `signal`?): `Promise`\<`void`\>

Prune tracking refs that are no longer present on remote

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `options`? | `null` \| [`PruneOptions`](../interfaces/PruneOptions.md) |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`void`\>

***

### defaultBranch()

> **defaultBranch**(`signal`?): `Promise`\<`string`\>

Get the remote’s default branch.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`string`\>
