# mergeCommits

두 커밋을 병합하여 병합 결과를 반영한 인덱스를 생성해요. 인덱스는 작업 디렉토리에 그대로 쓰거나 체크아웃할 수 있어요. 인덱스를 트리로 변환하려면 호출자가 병합 과정에서 발생한 충돌을 해결해야 해요.

## 시그니처

```ts
class Repository {
  mergeCommits(
    ourCommit: Commit,
    theirCommit: Commit,
    options?: MergeOptions | undefined | null,
  ): Index;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">ourCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">theirCommit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description"><code>ourCommit</code>에 병합할 커밋</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | MergeOptions</span>
    <br>
    <p class="param-description">병합 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">diff3 스타일 파일 생성</p>
      </li>
      <li class="param-li">
        <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌이 발생하면 계속 충돌을 해결하려고 시도하는 대신 즉시 종료</p>
      </li>
      <li class="param-li">
        <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
        <br>
        <p class="param-description">충돌 해결을 위해 선호할 측면 지정</p>
      </li>
      <li class="param-li">
        <span class="param-name">findRenames</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">파일 이름 변경 감지</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">모든 공백 무시</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">공백 양의 변경 무시</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">줄 끝의 공백 무시</p>
      </li>
      <li class="param-li">
        <span class="param-name">minimal</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">최소 diff를 찾기 위해 추가 시간 소요</p>
      </li>
      <li class="param-li">
        <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">병합되는 커밋에 여러 병합 베이스가 있는 경우, 재귀적 병합 베이스를 구축하지 않고(여러 병합 베이스를 병합하여) 단순히 첫 번째 베이스를 사용해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">patience</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">&quot;patience diff&quot; 알고리즘 사용</p>
      </li>
      <li class="param-li">
        <span class="param-name">recursionLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">크리스크로스 병합에 직면했을 때 가상 병합 베이스를 구축하기 위해 공통 조상을 병합하는 최대 횟수. 이 제한에 도달하면 다음 조상을 병합하려고 시도하는 대신 단순히 사용해요. 기본값은 무제한이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 이름이 변경된 것으로 간주할 유사도 (기본값 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">단순화된 diff 파일을 위해 영숫자가 아닌 영역 압축</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">생성된 인덱스에 REUC 확장을 작성하지 않음</p>
      </li>
      <li class="param-li">
        <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">표준 충돌 병합 파일 생성</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">이름 변경을 위해 검사할 최대 유사성 소스 (기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값보다 크면 부정확한 이름 변경 감지가 중단되어요. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의해요.</p>
      </li>
    </ul>
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