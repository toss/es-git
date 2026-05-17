# id

이 스태시 항목의 id (SHA1)를 가져와요.

각 스태시는 커밋 개체로 저장되며, 이 메서드는 커밋 SHA를 반환해요.

## 시그니처

```ts
class StashEntry {
  id(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">스태시 커밋의 40자 16진수 SHA1 해시.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const stashList = repo.stashList();
const stash = stashList.get(0);
console.log(stash?.id()); // e.g., "a1b2c3d4e5f6..."
```