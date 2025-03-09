[es-git](../globals.md) / Reference

# 클래스: Reference

Git [레퍼런스(reference)][1]를 나타내는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-Refs

## 메소드

### delete()

> **delete**(): `void`

기존 레퍼런스를 삭제해요.

이 메서드는 직접적인(direct) 레퍼런스와 심볼릭(symbolic) 레퍼런스 모두에서 작동해요. 레퍼런스는 디스크에서 즉시 삭제돼요.

레퍼런스가 조회된 이후 변경된 경우, 이 함수는 에러를 반환해요.

#### 반환 형식:

`void`

***

### isBranch()

> **isBranch**(): `boolean`

레퍼런스가 로컬 브랜치인지 확인해요.

#### 반환 형식:

`boolean`

***

### isNote()

> **isNote**(): `boolean`

레퍼런스가 노트인지 확인해요.

#### 반환 형식:

`boolean`

***

### isRemote()

> **isRemote**(): `boolean`

레퍼런스가 리모트인지 확인해요.

#### 반환 형식:

`boolean`

***

### isTag()

> **isTag**(): `boolean`

레퍼런스가 태그인지 확인해요.

#### 반환 형식:

`boolean`

***

### type()

> **type**(): `null` \| [`ReferenceType`](../type-aliases/ReferenceType.md)

레퍼런스의 타입을 가져와요.

타입이 알려지지 않은 경우 `null`을 반환해요.

#### 반환 형식:

`null` \| [`ReferenceType`](../type-aliases/ReferenceType.md)

***

### name()

> **name**(): `string`

레퍼런스의 전체 이름을 가져와요.

이름이 유효한 UTF-8이 아닌 경우 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### shorthand()

> **shorthand**(): `string`

레퍼런스의 축약 이름을 가져와요.

이 메서드는 레퍼런스 이름을 "사람이 읽을 수 있는" 형태로 변환해요. 적절한 축약 이름이 없는 경우 전체 이름을 반환해요.

축약 이름이 유효한 UTF-8이 아닌 경우 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### target()

> **target**(): `null` \| `string`

직접적인 레퍼런스(direct reference)가 가리키는 OID를 가져와요.

이 메서드는 레퍼런스가 직접적인 경우에만 사용할 수 있어요. (즉, 객체 ID 참조이며 심볼릭 레퍼런스가 아님)

#### 반환 형식:

`null` \| `string`

***

### targetPeel()

> **targetPeel**(): `null` \| `string`

이 레퍼런스를 추적(peel)해 OID를 반환해요.

이 OID는 직접적인 레퍼런스 중 하드 태그 객체를 가리키는 레퍼런스에만 적용돼요.

#### 반환 형식:

`null` \| `string`

***

### peelToTree()

> **peelToTree**(): [`Tree`](Tree.md)

레퍼런스를 트리(tree)가 발견될 때까지 재귀적으로 추적해요.

#### 반환 형식:

[`Tree`](Tree.md)

***

### symbolicTarget()

> **symbolicTarget**(): `null` \| `string`

심볼릭 레퍼런스가 가리키는 레퍼런스의 전체 이름을 가져와요.

이 메서드는 레퍼런스가 심볼릭인 경우에만 사용할 수 있어요.

#### 반환 형식:

`null` \| `string`

***

### resolve()

> **resolve**(): [`Reference`](Reference.md)

심볼릭 레퍼런스를 직접적인 레퍼런스로 반환해요.

이 메서드는 심볼릭 레퍼런스를 추적해 OID에 대한 직접적인 레퍼런스로 변환해요.

#### 반환 형식:

[`Reference`](Reference.md)

***

### rename()

> **rename**(`newName`, `options`?): [`Reference`](Reference.md)

기존 레퍼런스의 이름을 변경해요.

이 메서드는 직접적인 레퍼런스와 심볼릭 레퍼런스 모두에서 작동해요.

`force` 옵션이 `true`가 아니고 동일한 이름을 가진 레퍼런스가 이미 존재하는 경우 이름 변경은 실패해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `newName` | `string` |
| `options`? | `null` \| [`RenameReferenceOptions`](../interfaces/RenameReferenceOptions.md) |

#### 반환 형식:

[`Reference`](Reference.md)
