# revertCommit

주어진 "our" 커밋에 대해 주어진 커밋을 되돌려서 되돌리기 결과를 반영하는 인덱스를 생성해요.

변경 사항을 적용하려면 반환된 인덱스를 디스크에 써야 해요.

## 시그니처

```ts
class Repository {
  revertCommit(
    revertCommit: Commit,
    ourCommit: Commit,
    mainline: number,
    mergeOptions?: MergeOptions | undefined | null,
  ): Index;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">revertCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">되돌릴 커밋.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ourCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">되돌리기의 기준이 되는 커밋 (보통 HEAD).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mainline</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">되돌릴 커밋이 병합 커밋인 경우, 해당 부모 커밋 (1부터 시작).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mergeOptions</span><span class="param-type">MergeOptions | null</span>
    <br>
    <p class="param-description">병합 충돌 해결 옵션.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">diff3 스타일 파일 생성 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌 발생 시 충돌 해결을 계속 시도하지 않고 즉시 종료 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
        <br>
        <p class="param-description">충돌 해결 시 우선할 측면 지정.</p>
      </li>
      <li class="param-li">
        <span class="param-name">findRenames</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">파일 이름 변경 감지 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">모든 공백 무시 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">공백 양의 변경 무시 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">줄 끝 공백 무시 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">minimal</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">최소 diff를 찾기 위해 추가 시간 투입 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">병합할 커밋에 여러 병합 베이스가 있는 경우, 여러 병합 베이스를 병합하여 재귀적 병합 베이스를 구성하지 않고 단순히 첫 번째 베이스를 사용 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">patience</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">&quot;patience diff&quot; 알고리즘 사용 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">recursionLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">크리스크로스 병합 시 가상 병합 베이스를 구성하기 위해 공통 조상을 병합하는 최대 횟수. 이 한도에 도달하면 다음 조상은 병합 시도 없이 그대로 사용돼요. 기본값은 무제한.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 이름 변경으로 간주하는 유사도 (기본값 50).</p>
      </li>
      <li class="param-li">
        <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">간소화된 diff 파일을 위해 영숫자가 아닌 영역 압축 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">생성된 인덱스에 REUC 확장을 쓰지 않을지 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">표준 충돌 병합 파일 생성 여부.</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">이름 변경 감지를 위해 검사할 최대 유사도 소스 수 (기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값보다 크면 부정확한 이름 변경 감지가 중단돼요. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">인덱스 결과.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const head = repo.head().target()!;
const our = repo.getCommit(head);
const target = repo.getCommit(head);

// Compute a revert index and apply to working tree
const idx = repo.revertCommit(target, our, 0);
repo.checkoutIndex(idx);
```