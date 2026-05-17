# stashDrop

스태시 목록에서 단일 스태시 상태를 제거해요.

이 작업은 스태시 항목을 영구적으로 삭제해요. 스태시는 목록에서 제거되며
복구할 수 없어요. 이후의 모든 스태시는 재인덱싱돼요 (예: stash@{1}을 삭제하면
stash@{2}가 stash@{1}이 돼요).

## 시그니처

```ts
class Repository {
  stashDrop(index: number): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">삭제할 스태시의 인덱스 (0이 가장 최근 항목).</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">스태시 인덱스가 유효하지 않은 경우.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Drop the most recent stash
repo.stashDrop(0);

// Drop the third stash
repo.stashDrop(2);
```