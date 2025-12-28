# cherrypickCommit

`ourCommit`에 대해 `cherrypickCommit`의 체리픽을 적용하고, 그 결과로 생성되는 Index를 반환해요.
작업 디렉터리나 리포지토리 상태는 수정하지 않아요.
이 메서드는 디스크에 어떤 변경도 기록하지 않고 HEAD도 업데이트하지 않아요.
실제로 적용하지 않고도 체리픽 결과가 어떻게 보일지 계산하는 데 유용해요.

## 시그니처

```ts
class Repository {
  cherrypickCommit(
    cherrypickCommit: Commit,
    ourCommit: Commit,
    mainline: number,
    mergeOptions?: MergeOptions | undefined | null,
  ): Index;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">cherrypickCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">체리픽할 커밋</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ourCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">체리픽을 적용할 대상 커밋(보통 HEAD)</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mainline</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">체리픽 커밋이 머지인 경우 사용할 부모(1부터 시작)</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mergeOptions</span><span class="param-type">null | MergeOptions</span>
    <br>
    <p class="param-description">머지 충돌 해결을 위한 옵션</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">인덱스 결과</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">체리픽 커밋이 머지인데 mainline이 0인 경우예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">충돌이 있고 failOnConflict가 true(기본값)인 경우예요.</p>
  </li>
</ul>

## 예제

```ts
// This is an example for cherrypick_commit
import { openRepository } from "es-git";

const repo = await openRepository("./path/to/repo");
const cherry = repo.getCommit("cherrypick-commit");
const target = repo.getCommit("onto-commit");

// Returns the Index resulting from the cherrypick in memory,
// without affecting HEAD or the working tree.
// The mainline parameter indicates which parent to use as the baseline,
// For merge commits, mainline specifies which parent to use as baseline (1 or 2).
// For normal (non-merge) commits, use mainline 0.
const idx = repo.cherrypickCommit(cherry, target, 0);

// You can check for conflicts with idx.hasConflicts()
```