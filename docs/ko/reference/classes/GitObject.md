[es-git](../globals.md) / GitObject

# 클래스: GitObject

Git [개체(Object)][1]를 표현하는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### id()

> **id**(): `string`

저장소 개체의 ID(SHA1)를 가져와요.

#### 반환 형식:

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

개체의 타입을 가져와요.

타입이 알려지지 않은 경우 `null`을 반환해요.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### peel()

> **peel**(`objType`): [`GitObject`](GitObject.md)

지정된 타입의 개체가 나올 때까지 개체를 재귀적으로 추적(peel)해요.

`Any`를 대상 타입으로 전달하면, 타입이 변경될 때까지 개체를 추적해요. 예를 들어, 태그는 참조된 개체가 더 이상 태그가 아닐 때까지 추적돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### peelToCommit()

> **peelToCommit**(): [`Commit`](Commit.md)

개체를 커밋이 발견될 때까지 재귀적으로 추적해요.

#### 반환 형식:

[`Commit`](Commit.md)

***

### peelToBlob()

> **peelToBlob**(): [`Blob`](Blob.md)

개체를 블롭이 발견될 때까지 재귀적으로 추적해요.

#### 반환 형식:

[`Blob`](Blob.md)

***

### asCommit()

> **asCommit**(): `null` \| [`Commit`](Commit.md)

이 개체를 커밋으로 간주하려 시도해요.

개체가 실제로 커밋이 아닌 경우 `null`을 반환해요.

#### 반환 형식:

`null` \| [`Commit`](Commit.md)
