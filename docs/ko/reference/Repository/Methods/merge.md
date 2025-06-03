# merge

주어진 커밋을 HEAD에 병합하고 결과를 작업 디렉토리에 써요. 모든 변경사항은 커밋을 위해 스테이징되고 모든 충돌은 인덱스에 기록돼요. 호출자는 이 작업이 완료된 후 리포지토리의 인덱스를 검사하고, 충돌을 해결하고, 커밋을 준비해야 해요.

git과의 호환성을 위해 리포지토리는 병합 상태가 돼요. 커밋이 완료되면 (또는 사용자가 중단하려는 경우) `cleanupState()`를 호출하여 이 상태를 지워야 해요.

## 시그니처

```ts
class Repository {
  merge(
    annotatedCommits: AnnotatedCommit[],
    mergeOptions?: MergeOptions | undefined | null,
    checkoutOptions?: CheckoutOptions | undefined | null,
  ): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">annotatedCommits</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit[]</span>
    <br>
    <p class="param-description">병합할 커밋</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">mergeOptions</span><span class="param-type">null | MergeOptions</span>
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
        <p class="param-description">충돌이 발생하면 충돌 해결을 계속 시도하는 대신 즉시 종료</p>
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
        <p class="param-description">최소 diff를 찾기 위해 추가 시간 사용</p>
      </li>
      <li class="param-li">
        <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">병합되는 커밋에 여러 병합 베이스가 있는 경우, (여러 병합 베이스를 병합하여) 재귀적 병합 베이스를 구축하지 않고 단순히 첫 번째 베이스를 사용</p>
      </li>
      <li class="param-li">
        <span class="param-name">patience</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">&quot;patience diff&quot; 알고리즘 사용</p>
      </li>
      <li class="param-li">
        <span class="param-name">recursionLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">크리스크로스 병합에 직면했을 때 가상 병합 베이스를 구축하기 위해 공통 조상을 병합하는 최대 횟수. 이 제한에 도달하면 다음 조상을 병합하려고 시도하는 대신 단순히 사용. 기본값은 무제한</p>
      </li>
      <li class="param-li">
        <span class="param-name">renameThreshold</span><span class="param-type">number</span>
        <br>
        <p class="param-description">파일 이름이 변경된 것으로 간주할 유사성 (기본값 50)</p>
      </li>
      <li class="param-li">
        <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">단순화된 diff 파일을 위해 비영숫자 영역 압축</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">생성된 인덱스에 REUC 확장을 쓰지 않음</p>
      </li>
      <li class="param-li">
        <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">표준 충돌 병합 파일 생성</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetLimit</span><span class="param-type">number</span>
        <br>
        <p class="param-description">이름 변경을 위해 검사할 최대 유사성 소스 (기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값보다 크면 부정확한 이름 변경 감지가 중단됨. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">checkoutOptions</span><span class="param-type">null | CheckoutOptions</span>
    <br>
    <p class="param-description">체크아웃 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">안전 모드에서 체크아웃을 취소하는 대신 충돌이 있더라도 안전한 파일 업데이트를 적용. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 공통 조상 측면 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌에 대한 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부를 나타냄. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌에 대해 일반 병합 파일을 작성해야 하는지 나타냄. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">dirPerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">새 디렉토리가 생성되는 모드 설정. 기본값은 0755</p>
      </li>
      <li class="param-li">
        <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">CRLF 변환과 같은 필터를 적용할지 여부를 나타냄</p>
      </li>
      <li class="param-li">
        <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>path</code>에 지정된 경로를 pathspec이 아닌 정확한 파일 경로로 처리</p>
      </li>
      <li class="param-li">
        <span class="param-name">dryRun</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 체크아웃이 충돌을 확인하지만 실제 변경은 하지 않는 드라이런을 수행해야 함을 나타냄</p>
      </li>
      <li class="param-li">
        <span class="param-name">filePerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">새 파일이 생성되는 모드 설정. 기본값은 blob에 의해 지정된 0644 또는 0755</p>
      </li>
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">수정된 파일을 잠재적으로 삭제하는 것을 포함하여 작업 디렉토리를 대상과 일치시키기 위해 필요한 모든 조치를 취함</p>
      </li>
      <li class="param-li">
        <span class="param-name">ourLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 우리 측면 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃 중에 무시된 파일을 덮어써야 하는지 나타냄. 기본값은 true</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">체크아웃할 경로 추가. 경로는 <code>disablePathspecMatch</code>가 설정되지 않는 한 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴. 경로가 지정되지 않으면 모든 파일이 체크아웃됨. 그렇지 않으면 지정된 경로만 체크아웃됨</p>
      </li>
      <li class="param-li">
        <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">안전 모드에서 존재하지 않는 파일 생성. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">refresh</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 전에 인덱스와 git 속성을 디스크에서 새로 고쳐야 하는지 나타냄. 기본값은 true</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉토리에서 무시된 파일 제거. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉토리에서 추적되지 않은 파일 제거. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">safe</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 안전하게 수행되어야 함을 나타냄. 새 파일 생성은 허용하지만 기존 파일이나 변경사항을 덮어쓰지 않음. 이것이 기본값</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">병합되지 않은 인덱스 항목이 있는 파일 건너뛰기. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetDir</span><span class="param-type">string</span>
        <br>
        <p class="param-description">체크아웃할 디렉토리 설정</p>
      </li>
      <li class="param-li">
        <span class="param-name">theirLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 그들 측면 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 업데이트된 파일의 정보를 인덱스에 쓰는 것을 방지. 기본값은 true</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이미 존재하는 파일의 내용만 업데이트. 설정되면 파일이 생성되거나 삭제되지 않음. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">useOurs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 파일의 스테이지 2 버전(&quot;우리 것&quot;)을 사용하여 충돌을 진행해야 하는지 나타냄. 기본값은 false</p>
      </li>
      <li class="param-li">
        <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 파일의 스테이지 3 버전(&quot;그들 것&quot;)을 사용하여 충돌을 진행해야 하는지 나타냄. 기본값은 false</p>
      </li>
    </ul>
  </li>
</ul>