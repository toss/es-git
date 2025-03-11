[es-git](../globals.md) / DiffFile

# 클래스: DiffFile

delta의 한 쪽을 설명해요.

비록 "file"이라고 불리지만, 이는 파일, 심볼릭 링크, 서브모듈 커밋 ID, 또는 트리(단, 타입 변경이나 무시/추적되지 않은 디렉터리를 추적하는 경우에 한함)를 나타낼 수도 있어요.

## 메소드

### id()

> **id**(): `string`

해당 항목의 Oid를 반환해요.

만약 이 항목이 diff의 존재하지 않는 쪽을 나타낸다면 (예: `Added` delta의 `oldFile`), 반환되는 oid는 0으로 채워져 있어요.

#### 반환 형식:

`string`

***

### path()

> **path**(): `null` \| `string`

리포지토리의 작업 디렉터리를 기준으로 한 항목의 경로를 반환해요.

#### 반환 형식:

`null` \| `string`

***

### size()

> **size**(): `bigint`

해당 항목의 크기를 바이트 단위로 반환해요.

#### 반환 형식:

`bigint`

***

### isBinary()

> **isBinary**(): `boolean`

파일이 바이너리 데이터로 처리되는 경우, `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### isValidId()

> **isValidId**(): `boolean`

`id` 값이 올바른 것으로 알려져 있으면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### exists()

> **exists**(): `boolean`

해당 쪽의 파일이 존재하면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### mode()

> **mode**(): [`FileMode`](../type-aliases/FileMode.md)

파일 모드를 반환해요.

#### 반환 형식:

[`FileMode`](../type-aliases/FileMode.md)
