[es-git](../globals.md) / Remote

# 클래스: Remote

Git 리포지토리의 [리모트(Remote)][1]를 나타내는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%ea%b8%b0%ec%b4%88-%eb%a6%ac%eb%aa%a8%ed%8a%b8-%ec%a0%80%ec%9e%a5%ec%86%8c

## 메소드

### name()

> **name**(): `null` \| `string`

리모트의 이름을 가져와요.

리모트가 아직 이름이 지정되지 않은 경우 `null`을 반환하며, 이름이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### url()

> **url**(): `string`

리모트의 URL을 가져와요.

URL이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### pushurl()

> **pushurl**(): `null` \| `string`

리모트의 푸시 URL을 가져와요.

푸시 URL이 존재하지 않으면 `null`을 반환하며, URL이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### refspecs()

> **refspecs**(): [`Refspec`](../interfaces/Refspec.md)[]

모든 refspec들을 나열해요.

utf-8 형식으로 유효하지 않은 `src`나 `dst`가 있는 refspec들은 필터링돼요.

#### 반환 형식:

[`Refspec`](../interfaces/Refspec.md)[]

***

### fetch()

> **fetch**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

새로운 데이터를 다운로드하고 최신 상태로 업데이트해요.

리모트에 연결하여 데이터를 다운로드한 후 연결을 끊고 원격 추적 브랜치들을 업데이트하는 편리한 함수예요.

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

푸시를 수행해요.

푸시를 위한 모든 단계를 수행해요. 만약 refspecs가 전달되지 않으면, 설정된 refspecs가 사용돼요.

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

리모트에 더 이상 존재하지 않는 원격 추적 ref들을 정리해요.

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

리모트의 기본 브랜치를 가져와요.

리모트에 대한 `fetch` 작업도 함께 수행돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`string`\>
