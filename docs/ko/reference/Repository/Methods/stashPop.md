# stashPop

스태시 목록에서 단일 스태시 상태를 적용하고, 성공하면 목록에서 제거해요.

이 메서드는 `stashApply`와 `stashDrop`을 하나의 작업으로 결합해요. 스태시를 작업 디렉터리에 적용하고, 성공하면 스태시 목록에서 제거해요.
적용에 실패한 경우(예: 충돌로 인해), 스태시는 목록에 남아 있어요.

## 시그니처

```ts
class Repository {
  stashPop(index: number, options?: StashApplyOptions): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">팝할 스태시의 인덱스 (0이 가장 최근).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">StashApplyOptions | null</span>
    <br>
    <p class="param-description">스태시 적용 옵션.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">reinstantiateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">스태시에서 인덱스를 재설치할지 여부. true이면 스태시에 기록된 인덱스 상태도 복원돼요. 기본값: false</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">스태시 인덱스가 유효하지 않거나 적용 중 충돌이 발생한 경우.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Pop the most recent stash
repo.stashPop(0);

// Pop with options
repo.stashPop(0, { reinstantiateIndex: true });
```