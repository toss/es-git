# refspecs

모든 refspec 목록을 가져와요.

유효한 UTF-8 형식이 아닌 `src` 또는 `dst` 값을 가진 refspec은 필터링돼요.

## 시그니처

```ts
class Remote {
  refspecs(): Refspec[];
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Refspec[]</span>
    <br>
    <p class="param-description">모든 refspec 목록을 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// 리모트에 설정된 refspec 목록을 가져와요.
const refspecs = remote.refspecs();
console.log(refspecs[0]);
// "+refs/heads/*:refs/remotes/origin/*" refspec 예시
// {
//   "direction": "Fetch",
//   "src": "refs/heads/*",
//   "dst": "refs/remotes/origin/*",
//   "force": true
// }
```