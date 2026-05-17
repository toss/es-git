# message

이 스태시 항목의 메시지를 가져와요.

스태시가 생성될 때 연결된 메시지를 반환해요. 사용자 지정 메시지가 제공되지 않은 경우 Git에서 생성한 기본 메시지를 반환해요.

## 시그니처

```ts
class StashEntry {
  message(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">스태시 메시지, 또는 사용할 수 없는 경우 null.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();
const stash = stashList.get(0);
console.log(stash?.message()); // e.g., "WIP on main: abc1234 fix: typo"
```