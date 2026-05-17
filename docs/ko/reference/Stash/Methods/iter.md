# iter

이 목록의 스태시 항목에 대한 이터레이터를 반환해요.

이터레이터는 가장 최근 항목(인덱스 0)부터 가장 오래된 항목 순으로 스태시 항목을 생성해요.

## 시그니처

```ts
class StashList {
  iter(): StashListIter;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashListIter</span>
    <br>
    <p class="param-description">StashEntry 객체를 생성하는 이터레이터.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

// 스태시 목록 순회
for (const stash of stashList.iter()) {
  console.log(`${stash.index()}: ${stash.message()}`);
}
```