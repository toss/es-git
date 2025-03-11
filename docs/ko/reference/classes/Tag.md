[es-git](../globals.md) / Tag

# 클래스: Tag

Git [태그(Tag)][1]를 나타내는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%ea%b8%b0%ec%b4%88-%ed%83%9c%ea%b7%b8

## 메소드

### id()

> **id**(): `string`

리포지토리 태그의 id (SHA1)를 가져와요.

#### 반환 형식:

`string`

***

### message()

> **message**(): `null` \| `string`

태그의 메시지를 가져와요.

메시지가 없거나 올바른 utf8 형식이 아니면 `null`을 반환해요.

#### 반환 형식:

`null` \| `string`

***

### name()

> **name**(): `string`

태그의 이름을 가져와요.

올바른 utf8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### peel()

> **peel**(): [`GitObject`](GitObject.md)

태그가 아닌 git 개체가 나타날 때까지 재귀적으로 태그를 풀어줘요.

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### tagger()

> **tagger**(): `null` \| [`Signature`](../interfaces/Signature.md)

태그 작성자를 가져와요.

작성자가 지정되지 않은 경우 `null`을 반환해요.

#### 반환 형식:

`null` \| [`Signature`](../interfaces/Signature.md)

***

### target()

> **target**(): [`GitObject`](GitObject.md)

태그된 개체를 가져와요.

이 메소드는 주어진 개체에 대해 리포지토리 조회를 수행하고 해당 개체를 반환해요.

#### 반환 형식:

[`GitObject`](GitObject.md)

***

### targetId()

> **targetId**(): `string`

태그된 개체의 OID를 가져와요.

#### 반환 형식:

`string`

***

### targetType()

> **targetType**(): `null` \| [`ObjectType`](../enumerations/ObjectType.md)

태그된 개체의 ObjectType을 가져와요.

#### 반환 형식:

`null` \| [`ObjectType`](../enumerations/ObjectType.md)
