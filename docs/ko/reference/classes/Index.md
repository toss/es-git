[es-git](../globals.md) / Index

# 클래스: Index

Git [인덱스(Index)][1]를 나타내는 클래스로 사용해요.

[1]: https://git-scm.com/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4

## 메소드

### version()

> **version**(): `number`

디스크에 기록된 인데스 버전을 가져와요.

유효한 반환 값은 2, 3 또는 4예요. 만약 3이 반환된다면, 확장 데이터가 필요 없을 경우 버전 2의 인덱스가 기록될 수도 있어요.

#### 반환 형식:

`number`

***

### setVersion()

> **setVersion**(`version`): `void`

디스크에 기록된 인덱스 버전을 설정해요.

유효한 값은 2, 3 또는 4예요. 만약 2가 주어지면, 인덱스를 정확하게 표현하기 위해 필요한 경우 git_index_write가 버전 3의 인덱스를 기록할 수도 있어요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `version` | `number` |

#### 반환 형식:

`void`

***

### getByPath()

> **getByPath**(`path`, `stage`?): `null` \| [`IndexEntry`](../interfaces/IndexEntry.md)

경로를 기준으로 인덱스 내의 항목 중 하나를 가져와요.

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

디스크상의 파일로부터 인덱스 항목을 추가 또는 업데이트해요.

파일 경로는 리포지토리의 작업 폴더에 상대적이어야 하며 읽을 수 있어야 해요.

bare 인덱스 인스턴스에서는 이 메소드가 실패할 거예요.

이 메소드는 gitignore 규칙을 무시하고 파일을 인덱스에 추가하도록 강제해요.

현재 이 파일이 병합 충돌의 결과라면, 더 이상 충돌 상태로 표시되지 않고, 충돌에 관한 데이터는 "resolve undo" (REUC) 영역으로 이동돼요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `path` | `string` |

#### 반환 형식:

`void`

***

### addAll()

> **addAll**(`pathspecs`, `options`?): `void`

작업 디렉터리 내의 파일과 일치하는 인덱스 항목들을 추가 또는 업데이트해요.

bare 인덱스 인스턴스에서는 이 메소드가 실패할 거예요.

`pathspecs`는 리포지토리의 작업 디렉터리의 파일들과 매칭될 파일 이름 또는 쉘 glob 패턴의 목록이에요. 매칭되는 각 파일은 인덱스에 추가돼요(기존 항목은 업데이트되거나, 새 항목이 추가돼요).


#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `pathspecs` | `string`[] |
| `options`? | `null` \| [`IndexAddAllOptions`](../interfaces/IndexAddAllOptions.md) |

#### 반환 형식:

`void`

#### Example

`git add *`를 수행:

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

하드 디스크에서 읽어 메모리 내에 존재하는 인덱스 개체의 내용을 업데이트해요.

만약 force가 true로 설정되면, 메모리상의 변경 사항을 모두 버리고 항상 디스크상의 인덱스 데이터를 다시 로드하는 "강제" 읽기를 수행해요. 디스크상에 version 데이터가 없으면 인덱스는 초기화돼요.

force가 false면, 마지막 로드 이후에 디스크상의 데이터가 변경되었을 경우에만 인덱스 데이터를 재로딩하는 "소프트" 읽기를 수행해요. 메모리상의 순수 데이터는 그대로 유지돼요. 주의할 점은, 디스크에 변경 사항이 있을 경우 메모리상의 기록되지 않은 변경 사항은 버려진다는 거예요.

#### 매개변수

| 매개변수 | 유형 |
| ------ | ------ |
| `force`? | `null` \| `boolean` |

#### 반환 형식:

`void`

***

### write()

> **write**(): `void`

메모리 내의 기존 인덱스 개체를 원자적 파일 잠금을 사용하여 디스크에 다시 기록해요.

#### 반환 형식:

`void`

***

### writeTree()

> **writeTree**(): `string`

인덱스를 트리 형태로 기록해요.

이 메소드는 인덱스를 검색하여 현재 상태를 디스크에 기록해요; 인덱스에 저장된 각 서브트리에 대해 재귀적으로 트리 개체를 생성하지만, 최종적으로는 루트 트리의 OID만 반환해요. 이 OID는 예를 들어 커밋을 생성할 때 사용될 수 있어요.

인덱스 인스턴스는 bare이면 안 되며, 기존 리포지토리와 연결되어 있어야 해요.

인덱스에는 충돌 상태인 파일이 없어야 해요.


#### 반환 형식:

`string`

***

### removePath()

> **removePath**(`path`, `options`?): `void`

디스크상의 파일에 해당하는 인덱스 항목을 제거해요.

파일 경로는 리포지토리의 작업 폴더에 상대적이어야 해요. 파일이 존재할 수도 있어요.

만약 이 파일이 현재 병합 충돌의 결과라면, 더 이상 충돌 상태로 표시되지 않아요. 충돌에 관한 데이터는 "resolve undo" (REUC) 섹션으로 이동돼요.

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

일치하는 모든 인덱스 항목들을 제거해요.

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

작업 디렉토리와 일치하도록 모든 인덱스 항목들을 업데이트해요.

이 메소드는 bare 인덱스 인스턴스에서는 실패할 거예요.

기존의 인덱스 항목들을 스캔하여 작업 디렉토리와 동기화하고, 해당 작업 디렉토리 파일이 더 이상 존재하지 않으면 삭제하며, 그렇지 않으면 정보를 업데이트해요 (필요한 경우 파일의 최신 버전을 ODB에 추가하는 것도 포함돼요).

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

현재 인덱스에 있는 항목들의 개수를 가져와요.

#### 반환 형식:

`number`

***

### isEmpty()

> **isEmpty**(): `boolean`

인덱스에 항목이 하나도 없으면 `true`를 반환해요.

#### 반환 형식:

`boolean`

***

### path()

> **path**(): `null` \| `string`

디스크상의 인덱스 파일 전체 경로를 가져와요.

만약 인메모리 인덱스라면 `null`을 반환해요.

#### 반환 형식:

`null` \| `string`

***

### hasConflicts()

> **hasConflicts**(): `boolean`

이 인덱스에 충돌이 있나요?

충돌이 포함되어 있으면 `true`, 그렇지 않으면 `false`를 반환해요.

#### 반환 형식:

`boolean`

***

### entries()

> **entries**(): [`IndexEntries`](IndexEntries.md)

이 인덱스에 있는 항목들을 순회할 수 있는 iterator를 가져와요.

#### 반환 형식:

[`IndexEntries`](IndexEntries.md)
