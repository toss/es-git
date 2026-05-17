# stashList

리포지토리의 스태시 상태 목록을 가져와요.

리포지토리의 모든 스태시에 접근할 수 있는 StashList 개체를 반환해요.
목록은 가장 최근 스태시가 인덱스 0에 오도록 정렬되어 있어요.

## 시그니처

```ts
class Repository {
  stashList(): StashList;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashList</span>
    <br>
    <p class="param-description">리포지토리의 모든 스태시 항목에 접근할 수 있는 컨테이너.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

if (!stashList.isEmpty()) {
  console.log(`Found ${stashList.len()} stashes`);
  for (const stash of stashList.iter()) {
    console.log(`${stash.index()}: ${stash.message()}`);
  }
}
```