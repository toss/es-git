[es-git](../globals.md) / Reference

# 클래스: Reference

Git [레퍼런스(Reference)][1]를 나타내는 클래스로 사용해요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-Refs

## 메소드

### delete()

> **delete**(): `void`

기존 레퍼런스를 삭제해요.

이 메소드는 직접 레퍼런스와 심볼릭 레퍼런스 모두에서 작동해요. 레퍼런스는 디스크에서 즉시 제거돼요.

레퍼런스가 조회된 이후 변경되었다면 이 함수는 오류를 반환해요.

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

레퍼런스가 리모트 추적 브랜치인지 확인해요.

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

타입을 알 수 없으면 `null`을 반환해요.

#### 반환 형식:

`null` \| [`ReferenceType`](../type-aliases/ReferenceType.md)

***

### name()

> **name**(): `string`

레퍼런스의 전체 이름을 가져와요.

이름이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### shorthand()

> **shorthand**(): `string`

레퍼런스의 전체 축약 이름을 가져와요.

이 메소드는 레퍼런스 이름을 사람이 읽기 좋은 버전으로 변환해요. 적절한 축약 이름이 없다면 전체 이름을 반환해요.

축약 이름이 올바른 utf-8 형식이 아니면 오류를 발생시켜요.

#### 반환 형식:

`string`

***

### target()

> **target**(): `null` \| `string`

직접 레퍼런스가 가리키는 OID를 가져와요.

이는 직접 레퍼런스(즉, 개체 ID를 참조하는 레퍼런스)인 경우에만 사용할 수 있어요.

#### 반환 형식:

`null` \| `string`

***

### targetPeel()

> **targetPeel**(): `null` \| `string`

이 레퍼런스의 풀린(peeled) OID 대상을 반환해요.

이 풀린 OID는 직접(direct) 레퍼런스에만 적용돼요.

#### 반환 형식:

`null` \| `string`

***

### peelToTree()

> **peelToTree**(): [`Tree`](Tree.md)

레퍼런스를 트리로 풀어요.

이 메소드는 트리에 도달할 때까지 레퍼런스를 재귀적으로 풀어요.

#### 반환 형식:

[`Tree`](Tree.md)

***

### symbolicTarget()

> **symbolicTarget**(): `null` \| `string`

심볼릭 레퍼런스가 가리키는 레퍼런스의 전체 이름을 가져와요.

이는 심볼릭 레퍼런스인 경우에만 사용할 수 있어요.

#### 반환 형식:

`null` \| `string`

***

### resolve()

> **resolve**(): [`Reference`](Reference.md)

심볼릭 레퍼런스를 직접 레퍼런스로 해석해요.

이 메소드는 심볼릭 레퍼런스를 반복적으로 풀어 OID에 대한 직접 레퍼런스로 변환해요.

직접 레퍼런스가 인자로 전달되면, 해당 레퍼런스의 복사본을 반환해요.

#### 반환 형식:

[`Reference`](Reference.md)

***

### rename()

> **rename**(`newName`, `options`?): [`Reference`](Reference.md)

기존 레퍼런스의 이름을 변경해요.

이 메소드는 직접 레퍼런스와 심볼릭 레퍼런스 모두에서 작동해요.

force 플래그가 활성화되어 있지 않고, 이미 해당 이름의 레퍼런스가 존재하면 이름 변경이 실패해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `newName` | `string` |
| `options`? | `null` \| [`RenameReferenceOptions`](../interfaces/RenameReferenceOptions.md) |

#### 반환 형식:

[`Reference`](Reference.md)
