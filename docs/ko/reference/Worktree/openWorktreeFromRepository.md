# openWorktreeFromRepository

리포지토리에서 워크트리를 열어요.

리포지토리가 워크트리인 경우 주어진 리포지토리와 연결된 워크트리를 열어요.

## 시그니처

```ts
function openWorktreeFromRepository(repo: Repository): Worktree;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">repo</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Repository</span>
    <br>
    <p class="param-description">워크트리를 열 리포지토리.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Worktree</span>
    <br>
    <p class="param-description">Worktree 인스턴스.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">리포지토리가 워크트리가 아니거나 열기에 실패한 경우 오류를 던져요.</p>
  </li>
</ul>

## 예제

리포지토리에서 워크트리를 열어요.

```ts
import { openRepository, openWorktreeFromRepository } from 'es-git';

const repo = await openRepository('.');
const worktree = openWorktreeFromRepository(repo);
```