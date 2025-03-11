[es-git](../globals.md) / GitObject

# 클래스: GitObject

Git [개체(Object)][1]을 나타내는 클래스로 사용해요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### id()

> **id**(): `string`

리포지토리 개체의 id (SHA1)를 가져와요.

#### 반환 형식:

`string`

***

### type()

> **type**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

개체의 타입을 가져와요.

타입을 알 수 없으면 `null`을 반환해요.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)

***

### peel()

> **peel**(`objType`): [`GitObject`](GitObject.md)

특정 타입의 개체가 나올 때까지 재귀적으로 개체를 풀어요.

만약 대상 타입으로 `Any`를 전달하면, 타입이 바뀔 때까지 (예: 태그인 경우 참조된 개체가 더 이상 태그가 아닐 때까지) 개체를 풀어요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `objType` | [`ObjectType`](../enumerations/ObjectType.md) |

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### peelToCommit()

> **peelToCommit**(): [`Commit`](Commit.md)

커밋이 나올 때까지 재귀적으로 개체를 풀어요.

#### 반환 형식:

[`Commit`](Commit.md)

***

### peelToBlob()

> **peelToBlob**(): [`Blob`](Blob.md)

블롭이 나올 때까지 재귀적으로 개체를 풀어요.

#### 반환 형식:

[`Blob`](Blob.md)

***

### asCommit()

> **asCommit**(): `null` \| [`Commit`](Commit.md)

이 개체를 커밋으로 간주하려고 시도해요.

만약 해당 개체가 실제 커밋이 아니라면 `null`을 반환해요.

#### 반환 형식:

`null` \| [`Commit`](Commit.md)
