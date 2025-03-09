[es-git](../globals.md) / DiffDelta

# 클래스: DiffDelta

Diff 엔트리에 대한 클래스예요.

## 메소드

### flags()

> **flags**(): `number`

이 delta에 대한 플래그를 반환해요.

자세한 내용은 `DiffFlags`의 문서를 참조하세요.

#### 반환 형식:

`number`

***

### numFiles()

> **numFiles**(): `number`

이 delta에 포함된 파일 수를 반환해요.

#### 반환 형식:

`number`

***

### status()

> **status**(): [`DeltaType`](../type-aliases/DeltaType.md)

이 엔트리의 상태를 반환해요.

#### 반환 형식:

[`DeltaType`](../type-aliases/DeltaType.md)

***

### oldFile()

> **oldFile**(): [`DiffFile`](DiffFile.md)

Diff의 "from" 측을 나타내는 파일을 반환해요.

여기서 "from" 측의 의미는 diff 생성에 사용된 함수에 따라 달라지며, 해당 함수의 문서에서 설명해요.

#### 반환 형식:

[`DiffFile`](DiffFile.md)

***

### newFile()

> **newFile**(): [`DiffFile`](DiffFile.md)

Diff의 "to" 측을 나타내는 파일을 반환해요.

여기서 "to" 측의 의미는 diff 생성에 사용된 함수에 따라 달라지며, 해당 함수의 문서에서 설명해요.

#### 반환 형식:

[`DiffFile`](DiffFile.md)
