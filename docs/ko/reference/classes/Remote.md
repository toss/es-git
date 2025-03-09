[es-git](../globals.md) / Remote

# 클래스: Remote

Git [리모트(remote)][1] 저장소를 나타내는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%ea%b8%b0%ec%b4%88-%eb%a6%ac%eb%aa%a8%ed%8a%b8-%ec%a0%80%ec%9e%a5%ec%86%8c

## 메소드

### name()

> **name**(): `null` \| `string`

리모트의 이름을 가져옵니다.

리모트 이름이 지정되지 않은 경우 `null`을 반환하고, 이름이 유효한 UTF-8이 아닐 경우 오류를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### url()

> **url**(): `string`

리모트의 URL을 가져옵니다.

URL이 유효한 UTF-8이 아닐 경우 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### pushurl()

> **pushurl**(): `null` \| `string`

리모트의 푸시(push) URL을 가져옵니다.

푸시 URL이 존재하지 않는 경우 `null`을 반환하며, URL이 유효한 UTF-8이 아닐 경우 오류를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### refspecs()

> **refspecs**(): [`Refspec`](../interfaces/Refspec.md)[]

리모트의 모든 Refspec을 가져와요.

유효한 UTF-8로 인코딩되지 않은 `src`나 `dst`가 있는 경우, 해당 Refspec은 목록에서 제외돼요.

#### 반환 형식:

[`Refspec`](../interfaces/Refspec.md)[]

***

### fetch()

> **fetch**(`refspecs`, `options`?, `signal`?): `Promise`\<`void`\>

새 데이터를 다운로드하고 브랜치의 최신 상태를 업데이트해요.

편리하게 원격에 연결하고 데이터를 다운로드한 후 연결을 끊고, 원격 추적 브랜치를 업데이트해요.

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

푸시(push) 작업을 수행해요.

푸시의 모든 단계를 실행합니다. `refspecs`가 전달되지 않으면 리모트에 설정된 refspecs가 사용돼요.

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

리모트에 더 이상 존재하지 않는 트래킹 참조를 정리(prune)해요.

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

리모트로부터 `fetch` 작업도 함께 수행돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `signal`? | `null` \| `AbortSignal` |

#### 반환 형식:

`Promise`\<`string`\>
