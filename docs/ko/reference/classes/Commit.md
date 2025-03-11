[es-git](../globals.md) / Commit

# 클래스: Commit

git 커밋을 나타내는 클래스로 사용해요.

## 메소드

### id()

> **id**(): `string`

리포지토리 커밋의 id (SHA1)를 가져와요.

#### 반환 형식:

`string`

***

### author()

> **author**(): [`Signature`](../interfaces/Signature.md)

이 커밋의 작성자(author)를 가져와요.

#### 반환 형식:

[`Signature`](../interfaces/Signature.md)

***

### committer()

> **committer**(): [`Signature`](../interfaces/Signature.md)

이 커밋의 커미터(committer)를 가져와요.

#### 반환 형식:

[`Signature`](../interfaces/Signature.md)

***

### message()

> **message**(): `string`

커밋의 전체 메시지를 가져와요.

반환되는 메시지는 잠재적인 선행 개행을 제거하여 약간 다듬어진 형태로 제공돼요.

메시지가 올바른 utf-8이 아닐 경우 에러를 발생시켜요.

#### 반환 형식:

`string`

***

### summary()

> **summary**(): `null` \| `string`

git 커밋 메시지의 간단한 "요약"을 가져와요.

반환되는 메시지는 커밋의 요약으로, 메시지의 첫 번째 단락을 공백을 제거하고 합친 형태로 제공돼요.

요약이 올바른 utf-8이 아닐 경우 에러를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### body()

> **body**(): `null` \| `string`

git 커밋 메시지의 긴 "본문"을 가져와요.

반환되는 메시지는 커밋의 본문으로, 메시지의 첫 번째 단락을 제외한 모든 내용을 포함하며, 선행 및 후행 공백이 제거돼요.

본문이 올바른 utf-8이 아닐 경우 에러를 발생시켜요.

#### 반환 형식:

`null` \| `string`

***

### time()

> **time**(): `Date`

커밋 시각(즉, 커미터의 시각)을 가져와요.

#### 반환 형식:

`Date`

***

### tree()

> **tree**(): [`Tree`](Tree.md)

커밋이 가리키는 트리를 가져와요.

#### 반환 형식:

[`Tree`](Tree.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

이 Commit을 `GitObject`로 사용할 수 있도록 캐스팅해요.

#### 반환 형식:

[`GitObject`](GitObject.md)
