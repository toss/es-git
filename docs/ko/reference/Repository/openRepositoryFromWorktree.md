# openRepositoryFromWorktree

워크트리에서 리포지토리를 열어요.

주어진 워크트리와 연결된 리포지토리를 열어요.

## 시그니처

```ts
function openRepositoryFromWorktree(worktree: Worktree): Repository;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">worktree</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Worktree</span>
    <br>
    <p class="param-description">리포지토리를 열 워크트리.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Repository</span>
    <br>
    <p class="param-description">리포지토리 인스턴스.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">리포지토리 열기에 실패하면 오류를 발생시켜요.</p>
  </li>
</ul>

## 예제

워크트리에서 리포지토리를 열어요.

```ts
import { openWorktreeFromRepository, openRepositoryFromWorktree } from 'es-git';

const worktree = await openWorktreeFromRepository(repo);
const repo = openRepositoryFromWorktree(worktree);
```