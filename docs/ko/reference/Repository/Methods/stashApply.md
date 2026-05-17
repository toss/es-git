# stashApply

스태시 목록에서 단일 스태시 상태를 적용해요.

이 메서드는 스태시 항목의 변경 사항을 작업 디렉터리에 적용해요.
`stashPop`과 달리, 적용 후 스태시 목록에서 스태시를 제거하지 않아요.
스태시된 변경 사항이 현재 작업 디렉터리와 충돌하는 경우 충돌이 발생할 수 있어요.

## 시그니처

```ts
class Repository {
  stashApply(index: number, options?: StashApplyOptions): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">적용할 스태시의 인덱스 (0이 가장 최근).</p>
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

// Apply the most recent stash
repo.stashApply(0);

// Apply with options
repo.stashApply(0, { reinstantiateIndex: true });
```