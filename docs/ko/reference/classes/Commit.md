[es-git](../globals.md) / Commit

# 클래스: Commit

Git 커밋을 나타내는 클래스예요.

## 메서드

### id()

> **id**(): `string`

리포지토리 커밋의 ID(SHA1 해시)를 가져와요.

#### 반환 값:

`string`

***

### author()

> **author**(): [`Signature`](../interfaces/Signature.md)

이 커밋의 작성자 정보를 가져와요.

#### 반환 값:

[`Signature`](../interfaces/Signature.md)

***

### committer()

> **committer**(): [`Signature`](../interfaces/Signature.md)

이 커밋의 커미터 정보를 가져와요.

#### 반환 값:

[`Signature`](../interfaces/Signature.md)

***

### message()

> **message**(): `string`

커밋의 전체 메시지를 가져와요.

반환된 메시지는 앞부분에 있을 수 있는 개행 문자를 제거하여 보기 좋게 정리돼요.  
메시지가 유효한 UTF-8 형식이 아니면 에러를 발생시켜요.

#### 반환 값:

`string`

***

### summary()

> **summary**(): `null` \| `string`

Git 커밋 메시지의 짧은 "요약"을 가져와요.

반환된 메시지는 커밋 메시지의 첫 번째 단락으로, 공백이 제거되고 정리된 형태예요.  
요약이 유효한 UTF-8 형식이 아니면 에러를 발생시켜요.

#### 반환 값:

`null` \| `string`

***

### body()

> **body**(): `null` \| `string`

Git 커밋 메시지의 긴 "본문"을 가져와요.

반환된 메시지는 첫 번째 단락을 제외한 나머지 부분으로, 앞뒤 공백이 제거돼요.  
본문이 유효한 UTF-8 형식이 아니면 에러를 발생시켜요.

#### 반환 값:

`null` \| `string`

***

### time()

> **time**(): `Date`

커밋 시간(커미터의 시간)을 가져와요.

반환된 값은 커밋 시간이 `Date` 객체로 제공돼요.

#### 반환 값:

`Date`

***

### tree()

> **tree**(): [`Tree`](Tree.md)

이 커밋이 가리키는 트리를 가져와요.

#### 반환 값:

[`Tree`](Tree.md)

***

### asObject()

> **asObject**(): [`GitObject`](GitObject.md)

이 커밋을 `GitObject`로 변환해서 사용할 수 있도록 해요.

#### 반환 값:

[`GitObject`](GitObject.md)
