[es-git](../globals.md) / DiffDelta

# 클래스: DiffDelta

하나의 항목에 대한 변경 사항을 설명해요.

## 메소드

### flags()

> **flags**(): `number`

delta에 설정된 플래그들을 반환해요.

자세한 내용은 `DiffFlags`의 문서를 참고하세요.

#### 반환 형식:

`number`

***

### numFiles()

> **numFiles**(): `number`

이 delta에 포함된 파일의 수를 반환해요.

#### 반환 형식:

`number`

***

### status()

> **status**(): [`DeltaType`](../type-aliases/DeltaType.md)

이 항목의 상태를 반환해요.

#### 반환 형식:

[`DeltaType`](../type-aliases/DeltaType.md)

***

### oldFile()

> **oldFile**(): [`DiffFile`](DiffFile.md)

diff에서 "from" 쪽을 나타내는 파일을 반환해요.

어느 쪽을 의미하는지는 diff를 생성하는 데 사용된 함수에 따라 달라지며, 해당 함수 자체의 문서에서 설명돼 있어요.

#### 반환 형식:

[`DiffFile`](DiffFile.md)

***

### newFile()

> **newFile**(): [`DiffFile`](DiffFile.md)

diff에서 "to" 쪽을 나타내는 파일을 반환해요.

어느 쪽을 의미하는지는 diff를 생성하는 데 사용된 함수에 따라 달라지며, 해당 함수 자체의 문서에서 설명돼 있어요.

#### 반환 형식:

[`DiffFile`](DiffFile.md)
