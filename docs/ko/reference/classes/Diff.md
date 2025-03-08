[es-git](../globals.md) / Diff

# 클래스: Diff

개별 파일 변경 사항(델타)을 포함하는 Diff 클래스예요.

이 구조는 `Repository` 클래스의 `diffTreeToTree` 같은 diff 생성 함수나 다른 `diff*` 함수로 할당돼요.  
직접 내부 구조를 다루지 않고, 제공된 메서드를 사용해서 diff 데이터를 처리해야 해요.

## 메서드

### merge()

> **merge**(`diff`): `void`

다른 diff를 현재 diff에 병합해요.

"from" 리스트에 있는 항목을 "self" 리스트로 병합해요.  
결과적으로 두 리스트에 있는 모든 항목이 포함되며,  
같은 항목이 두 리스트에 모두 존재하면 "onto" 리스트의 이전 버전과  
"from" 리스트의 새 버전이 합쳐져서 나타나요.  
단, 중간에 삭제(`DELETE`)가 포함된 경우에는 삭제된 것으로 표시돼요.

#### 매개변수

| 매개변수   | 유형                |
|--------|-------------------|
| `diff` | [`Diff`](Diff.md) |

#### 반환 형식:

`void`

***

### deltas()

> **deltas**(): [`Deltas`](Deltas.md)

이 diff에 포함된 변경 사항(델타)을 순회하는 반복자를 반환해요.

#### 반환 형식:

[`Deltas`](Deltas.md)

***

### isSortedIcase()

> **isSortedIcase**(): `boolean`

델타가 대소문자를 구분해서 정렬되었는지 확인해요.

#### 반환 형식:

`boolean`

***

### stats()

> **stats**(): [`DiffStats`](DiffStats.md)

모든 변경 사항(패치)에 대한 diff 통계를 계산해요.

#### 반환 형식:

[`DiffStats`](DiffStats.md)

***

### print()

> **print**(`options`?): `string`

diff를 순회하면서 서식이 지정된 텍스트 출력을 생성해요.

#### 매개변수

| 매개변수       | 유형                                                                |
|------------|-------------------------------------------------------------------|
| `options`? | `null` \| [`DiffPrintOptions`](../interfaces/DiffPrintOptions.md) |

#### 반환 형식:

`string`
