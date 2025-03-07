[es-git](../globals.md) / Blob

# 클래스: Blob

Git [blob][1]를 다루는 클래스예요.

[1]: http://git-scm.com/book/en/Git-Internals-Git-Objects

## 메소드

### id()

> **id**(): `string`

리포지토리 Blob의 ID(SHA1 해시)를 가져와요.

#### 반환 형식:

`string`

***

### isBinary()

> **isBinary**(): `boolean`

Blob의 내용이 바이너리 데이터인지 확인해요.

#### 반환 형식:

`boolean`

***

### content()

> **content**(): `Uint8Array`

이 Blob의 내용을 가져와요.

#### 반환 형식:

`Uint8Array`

***

### size()

> **size**(): `bigint`

이 Blob의 크기(바이트 단위)를 가져와요.

#### 반환 형식:

`bigint`
