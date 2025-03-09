[es-git](../globals.md) / Blob

# 클래스: Blob

Git [블롭(Blob)][1]를 다루는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### id()

> **id**(): `string`

리포지토리 블롭의 ID(SHA1 해시)를 가져와요.

#### 반환 형식:

`string`

***

### isBinary()

> **isBinary**(): `boolean`

블롭의 내용이 바이너리 데이터인지 확인해요.

#### 반환 형식:

`boolean`

***

### content()

> **content**(): `Uint8Array`

이 블롭의 내용을 가져와요.

#### 반환 형식:

`Uint8Array`

***

### size()

> **size**(): `bigint`

이 블롭의 크기(바이트 단위)를 가져와요.

#### 반환 형식:

`bigint`
