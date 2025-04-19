# branches

브랜치를 순회하는 반복자를 만들어요.

## 시그니처

```ts
class Repository {
  branches(filter?: BranchesFilter | null | undefined): Branches;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">filter</span><span class="param-type">null | BranchesFilter</span>
    <br>
    <p class="param-description">브랜치 반복자를 위한 필터에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">type</span><span class="param-type">BranchType</span>
        <br>
        <p class="param-description">필터할 브랜치 유형이에요.</p>
        <p class="param-description">- <code>Local</code> : 원격에 없는 로컬 브랜치에요.<br>- <code>Remote</code> : 원격 브랜치에요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branches</span>
    <br>
    <p class="param-description">브랜치를 순회하는 반복자</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');

for (const branch of repo.branches()) {
  console.log(branch.type); // "Local"
  console.log(branch.name); // "main"
}
```