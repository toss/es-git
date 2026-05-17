# get

지정된 인덱스에 있는 스태시 항목을 이 목록에서 가져와요.

## 시그니처

```ts
class StashList {
  get(index: number): StashEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">가져올 스태시 항목의 인덱스 (0부터 시작하며, 0이 가장 최근 항목).</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StashEntry | null</span>
    <br>
    <p class="param-description">지정된 인덱스에 있는 이 목록의 스태시 항목, 또는 인덱스가 범위를 벗어난 경우 <code>null</code>.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();

// 가장 최근 스태시 가져오기
const stash = stashList.get(0);
if (stash) {
  console.log(stash.message());
}
```