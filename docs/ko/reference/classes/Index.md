[es-git](../globals.md) / Index

# 클래스: Index

Git [인덱스(index)][1]를 표현하는 클래스예요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### version()

> **version**(): `number`

인덱스의 디스크 상 버전을 가져와요.

유효한 반환 값은 2, 3 또는 4입니다. 만약 3이 반환될 경우, 버전 3의 확장 데이터가 필요하지 않다면 버전 2로 인덱스를 작성할 수도 있어요.

#### 반환 형식:

`number`

***

### setVersion()

> **setVersion**(`version`): `void`

인덱스의 디스크 상 버전을 설정해요.

유효한 값은 2, 3 또는 4입니다. 2가 주어질 경우, 정확한 인덱스를 표현하기 위해 필요하다면 git_index_write가 버전 3의 인덱스를 작성할 수도 있어요.


#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `version` | `number` |

#### 반환 형식:

`void`

***

### getByPath()

> **getByPath**(`path`, `stage`?): `null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

경로를 기준으로 인덱스 내 항목 중 하나를 가져와요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |
| `stage`? | `null` \| [`IndexStage`](../type-aliases/IndexStage.md) |

#### 반환 형식:

`null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

***

### addPath()

> **addPath**(`path`): `void`

디스크에 있는 파일에서 인덱스 항목을 추가하거나 업데이트해요.

파일 경로는 저장소의 작업 폴더를 기준으로 상대적이어야 하며 읽기 가능해야 해요. 이 메소드는 bare 인덱스 인스턴스에서는 실패합니다.

이 메소드는 파일을 `gitignore` 규칙에 상관없이 강제로 인덱스에 추가해요.

파일이 현재 병합 충돌 결과인 경우, 해당 파일은 더 이상 충돌로 표시되지 않으며, 충돌에 대한 데이터는 "resolve undo"(REUC) 섹션으로 이동돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |

#### 반환 형식:

`void`

***

### addAll()

> **addAll**(`pathspecs`, `options`?): `void`

작업 디렉토리의 파일과 일치하는 인덱스 항목을 추가하거나 업데이트해요.

이 메소드는 bare 인덱스 인스턴스에서는 실패해요.

`pathspecs`는 저장소의 작업 디렉토리에 있는 파일과 일치할 파일 이름 또는 쉘 glob 패턴 목록입니다. 일치하는 각 파일은 인덱스에 추가되거나 기존 항목을 업데이트해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexAddAllOptions`](../interfaces/IndexAddAllOptions.md) |

#### 반환 형식:

`void`

#### Example

`git add *`와 동일하게 동작:

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
index.addAll(['*']);
index.write();
```

***

### read()

> **read**(`force`?): `void`

디스크에서 읽어 기존 인덱스 객체의 내용을 메모리에 업데이트해요.

`force`가 `true`인 경우, 이는 "하드(hard)" 읽기를 수행하여 메모리의 변경 사항을 삭제하고 디스크 상의 인덱스 데이터를 항상 새로고침해요. 디스크에 버전이 없을 경우, 인덱스가 초기화됩니다.

`force`가 `false`인 경우, 이는 "소프트(soft)" 읽기를 수행하여 마지막으로 로드된 이후에 디스크 데이터가 변경된 경우에만 데이터를 새로고칩됩니다. 순수하게 메모리에 있는 인덱스 데이터는 유지돼요. 디스크에 변경 사항이 있는 경우, 쓰지 않은 메모리 상의 변경 사항은 삭제돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `force`? | `null` \| `boolean` |

#### 반환 형식:

`void`

***

### write()

> **write**(): `void`

메모리에 존재하는 인덱스 객체를 디스크에 저장(쓰기)합니다. 이는 원자적 파일 잠금 방식을 사용해요.

#### 반환 형식:

`void`

***

### writeTree()

> **writeTree**(): `string`

인덱스를 트리 형식으로 작성해요.

이 메소드는 인덱스를 확인하여 현재 상태를 디스크에 작성해요. 인덱스에 저장된 각 하위 트리에 대해 재귀적으로 트리 객체를 생성하지만, 루트 트리의 OID만 반환합니다. 이는 예를 들어 커밋 생성에 사용할 수 있는 OID예요.

인덱스 인스턴스는 bare일 수 없으며, 기존 저장소와 연결되어야 해요. 또한 인덱스에는 충돌(conflict) 상태인 파일이 포함될 수 없어요.

#### 반환 형식:

`string`

***

### removePath()

> **removePath**(`path`, `options`?): `void`

디스크에 있는 파일에 해당하는 인덱스 항목을 제거해요.

파일 경로는 저장소의 작업 폴더를 기준으로 상대적이어야 합니다. 파일이 존재할 수도 있고, 존재하지 않을 수도 있어요.

만약 해당 파일이 병합 충돌 상태라면, 더 이상 충돌로 표시되지 않으며 충돌에 대한 데이터는 "resolve undo"(REUC) 섹션으로 이동돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |
| `options`? | `null` \| [`IndexRemoveOptions`](../interfaces/IndexRemoveOptions.md) |

#### 반환 형식:

`void`

***

### removeAll()

> **removeAll**(`pathspecs`, `options`?): `void`

일치하는 모든 인덱스 항목을 제거해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexRemoveAllOptions`](../interfaces/IndexRemoveAllOptions.md) |

#### 반환 형식:

`void`

***

### updateAll()

> **updateAll**(`pathspecs`, `options`?): `void`

인덱스 항목들을 작업 디렉토리와 일치하도록 모두 업데이트해요.

이 메서드는 bare 인덱스 인스턴스에서는 실패해요.

기존 인덱스 항목을 스캔하고 이를 작업 디렉토리와 동기화합니다. 작업 디렉토리에 파일이 더 이상 존재하지 않는다면 해당 항목을 삭제해요. 그렇지 않으면 정보를 업데이트하고, 필요 시 파일의 최신 버전을 ODB에 추가해요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexUpdateAllOptions`](../interfaces/IndexUpdateAllOptions.md) |

#### 반환 형식:

`void`

***

### count()

> **count**(): `number`

현재 인덱스에 있는 항목 수를 가져와요.

#### 반환 형식:

`number`

***

### isEmpty()

> **isEmpty**(): `boolean`

인덱스에 항목이 없으면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### path()

> **path**(): `null` \| `string`

디스크에 저장된 인덱스 파일의 전체 경로를 가져와요.

이 메서드는 메모리 상의 인덱스인 경우 `null`을 반환해요.

#### 반환 형식:

`null` \| `string`

***

### hasConflicts()

> **hasConflicts**(): `boolean`

현재 인덱스에 충돌이 있는지 확인해요.

인덱스에 충돌이 있으면 `true`, 없으면 `false`를 반환해요.

#### 반환 형식:

`boolean`

***

### entries()

> **entries**(): [`IndexEntries`](IndexEntries.md)

인덱스의 항목에 대한 반복자를 가져와요.

#### 반환 형식:

[`IndexEntries`](IndexEntries.md)
