[es-git](../globals.md) / DiffFile

# 클래스: DiffFile

Delta의 한 면에 대한 클래스예요.

이 클래스명이 "파일"이라 불리지만, 실제로는 파일, 심볼릭 링크, 하위 모듈 커밋 ID, 또는 트리를 나타낼 수 있어요. (다만, 이는 타입 변경 추적이나 무시된 혹은 추적되지 않은 디렉토리를 다룰 때 발생해요)

## 메소드

### id()

> **id**(): `string`

이 항목의 Oid를 반환해요.

이 항목이 delta의 없는 측(예: `Added` 델타의 `oldFile`)을 나타내는 경우, 반환된 Oid는 모두 0으로 표시돼요.

#### 반환 형식:

`string`

***

### path()

> **path**(): `null` \| `string`

저장소의 작업 디렉토리를 기준으로 엔트리의 경로를 반환해요.

#### 반환 형식:

`null` \| `string`

***

### size()

> **size**(): `bigint`

이 항목의 크기를 바이트 단위로 반환해요.

#### 반환 형식:

`bigint`

***

### isBinary()

> **isBinary**(): `boolean`

파일이 이진 데이터로 처리되면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### isValidId()

> **isValidId**(): `boolean`

`id` 값이 올바른 경우 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### exists()

> **exists**(): `boolean`

파일이 delta의 이 측에 존재하면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### mode()

> **mode**(): [`FileMode`](../type-aliases/FileMode.md)

파일 모드를 반환해요.

#### 반환 형식:

[`FileMode`](../type-aliases/FileMode.md)
