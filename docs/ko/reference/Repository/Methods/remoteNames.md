# remoteNames

리포지토리에 설정된 모든 리모트 목록을 가져와요.

## 시그니처

```ts
class Repository {
  remoteNames(): string[];
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">이 리포지토리에 설정된 모든 리모트의 이름을 배열로 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
console.log(repo.remoteNames()); // ["origin"]
```