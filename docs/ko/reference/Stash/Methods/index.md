# index

이 스태시 항목의 인덱스를 가져와요.

인덱스는 스태시 스택에서 이 스태시의 위치를 나타내며, 0이 가장 최근 스태시예요.

## 시그니처

```ts
class StashEntry {
  index(): number;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">이 스태시 항목의 인덱스 (0부터 시작하며, 0이 가장 최근).</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();
const stash = stashList.get(0);
console.log(stash?.index()); // 0
```