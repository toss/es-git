[es-git](../globals.md) / Diff

# 클래스: Diff

개별 파일 delta들을 모두 포함하는 diff 객체에요.

이는 불투명한 구조체로, `Repository` 클래스의 diff 생성 함수(예: `diffTreeToTree` 또는 다른 `diff*` 함수들)에 의해 할당돼요.

## 메소드

### merge()

> **merge**(`diff`): `void`

하나의 diff를 다른 diff에 병합해요.

이 메소드는 "from" 리스트의 항목들을 "self" 리스트에 병합해, 두 리스트 중 어느 한쪽에 나타나는 모든 항목들이 포함된 결과 diff를 만들어요.
만약 항목이 두 리스트 모두에 존재한다면, 해당 항목은 "onto" 리스트의 구버전과 "from" 리스트의 신버전이 병합된 것처럼 나타나요
(단, 해당 항목에 중간에 대기 중인 DELETE가 있으면 삭제된 것으로 표시돼요).

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `diff` | [`Diff`](Diff.md) |

#### 반환 형식:

`void`

***

### deltas()

> **deltas**(): [`Deltas`](Deltas.md)

이 diff 내에 포함된 delta들을 순회할 수 있는 iterator를 반환해요.

#### 반환 형식:

[`Deltas`](Deltas.md)

***

### isSortedIcase()

> **isSortedIcase**(): `boolean`

delta들이 대소문자 구분 또는 무시하여 정렬되었는지 확인해요.

#### 반환 형식:

`boolean`

***

### stats()

> **stats**(): [`DiffStats`](DiffStats.md)

모든 패치에 대한 diff 통계 정보를 집계해요.

#### 반환 형식:

[`DiffStats`](DiffStats.md)

***

### print()

> **print**(`options`?): `string`

포맷된 텍스트 출력을 생성하며 diff를 순회해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `options`? | `null` \| [`DiffPrintOptions`](../interfaces/DiffPrintOptions.md) |

#### 반환 형식:

`string`
