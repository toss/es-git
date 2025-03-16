# defaultBranch

리모트의 기본 브랜치를 가져와요.

이 메서드는 리모트에서 `fetch` 작업도 함께 수행해요.

## 시그니처

```ts
class Remote {
  defaultBranch(signal?: AbortSignal): Promise<string>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">요청을 중단할 때 사용할 <code>AbortSignal</code> 객체예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;string&gt;</span>
    <br>
    <p class="param-description">
      리모트의 기본 브랜치 이름을 반환해요.
    </p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

const branch = await remote.defaultBranch();
console.log(branch); // "refs/heads/main"
```