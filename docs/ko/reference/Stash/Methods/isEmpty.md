# isEmpty

스태시 목록이 비어 있는지 확인해요.

## 시그니처

```ts
class StashList {
  isEmpty(): boolean;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">이 리포지토리에 스태시 항목이 없으면 <code>true</code>를 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

if (stashList.isEmpty()) {
  console.log('No stashes found');
} else {
  console.log(`Found ${stashList.len()} stashes`);
}
```