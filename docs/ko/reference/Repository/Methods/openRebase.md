# openRebase

기존에 `rebase()` 호출이나 다른 클라이언트에 의해 시작된 리베이스를 엽니다.

## 시그니처

```ts
class Repository {
  openRebase(options?: RebaseOptions | undefined | null): Rebase;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RebaseOptions</span>
    <br>
    <p class="param-description">체크아웃 옵션, 병합 옵션, 메모리 내 리베이스와 같은 리베이스 동작에 대한 세부 조절.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutOptions</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description"><code>Repository::rebase</code>, <code>next()</code> 및 <code>abort()</code> 중 파일이 작성되는 방법을 제어하는 옵션입니다. <code>init</code> 및 <code>next</code>에서는 기본값으로 최소 전략인 <code>GIT_CHECKOUT_SAFE</code>가 설정되며, <code>abort</code>에서는 git 의미론에 맞추기 위해 기본값으로 최소 전략인 <code>GIT_CHECKOUT_FORCE</code>가 설정됩니다.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서, 취소하지 않고 충돌이 있어도 안전 파일 업데이트를 적용합니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 조상측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대한 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부를 나타냅니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 일반 병합 파일을 작성해야 하는지 여부를 나타냅니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리가 생성될 때 사용할 모드를 설정합니다. 기본값은 0755입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환과 같은 필터를 적용할지 여부를 나타냅니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에서 지정한 경로를 경로 명세가 아닌 정확한 파일 경로로 처리합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌을 확인하지만 실제로 변경하지 않도록 이 체크아웃이 드라이 런을 수행하도록 표시합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일이 생성될 때 사용할 모드를 설정합니다. 기본값은 blob에 의해 결정된 0644 또는 0755입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리가 변경된 파일을 폐기할 가능성을 포함하여 목표와 일치하도록 필요한 모든 작업을 수행합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 우리측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃 중에 무시된 파일을 덮어쓸 지 여부를 나타냅니다. 기본값은 true입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 경로를 추가합니다. <code>disablePathspecMatch</code>가 설정되지 않은 경우 경로는 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴입니다. 경로가 지정되지 않으면 모든 파일이 체크아웃됩니다. 그렇지 않으면 지정된 경로만 체크아웃됩니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성합니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">커밋 수행 전에 인덱스와 git 속성을 디스크에서 새로고침할지 여부를 나타냅니다. 기본값은 true입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">무시된 파일을 작업 디렉터리에서 제거합니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">추적되지 않은 파일을 작업 디렉터리에서 제거합니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">새 파일을 생성하지만 기존 파일이나 변경 사항을 덮어쓰지 않고 안전하게 체크아웃할지 여부를 나타냅니다. 기본값입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되지 않은 인덱스 항목이 있는 파일을 건너뜁니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 디렉터리를 설정합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 그들측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃 후 갱신된 파일의 정보를 인덱스에 기록하지 않도록 합니다. 기본값은 true입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트합니다. 설정된 경우 파일이 생성되거나 삭제되지 않습니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 발생 시 파일의 스테이지 2 버전(&quot;ours&quot;)으로 체크아웃을 진행할지 여부를 나타냅니다. 기본값은 false입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 발생 시 파일의 스테이지 3 버전(&quot;theirs&quot;)으로 체크아웃을 진행할지 여부를 나타냅니다. 기본값은 false입니다.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">inmemory</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션은 메모리 내 리베이스를 시작하고 호출자가 리베이스 작업을 단계별로 진행하고 리베이스된 변경 사항을 커밋할 수 있게 하지만 HEAD를 되돌리거나 리포지토리를 리베이스 상태로 업데이트하지 않습니다. 이는 작업 디렉터리(있을 경우)에 영향을 미치지 않습니다.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mergeOptions</span><span class="param-type">MergeOptions</span>
        <br>
        <p class="param-description"><code>next()</code>를 시행하는 동안 트리를 병합하는 방법을 제어하는 옵션입니다.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">diff3 스타일의 파일을 작성합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌이 발생하면 충돌 해결을 시도하지 않고 즉시 종료합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
            <br>
            <p class="param-description">충돌 해결을 위해 선택할 측을 지정합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">findRenames</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">파일 이름 변경을 감지합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">모든 공백을 무시합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">공백 양의 변화를 무시합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">줄 끝의 공백을 무시합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">minimal</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">최소 차이를 찾기 위해 추가 시간을 사용합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합된 커밋에 여러 병합 베이스가 있는 경우, 여러 병합 베이스를 병합하여 재귀적 병합 베이스를 생성하지 않고, 대신 첫 번째 베이스만 사용합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">patience</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">&quot;Patience diff&quot; 알고리즘을 사용합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recursionLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">십자형 병합이 있는 경우 가상 병합 베이스를 빌드하기 위해 공통 조상을 병합할 최대 횟수입니다. 이 한도에 도달하면 다음 조상이 병합 시도가 아닌 단순히 사용됩니다. 기본값은 무제한입니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">renameThreshold</span><span class="param-type">number</span>
            <br>
            <p class="param-description">파일이 이름이 바뀐 것으로 간주될 유사성(기본값 50)</p>
          </li>
          <li class="param-li">
            <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">단순화된 차이 파일을 위해 비알파벳 문자 영역을 압축합니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">생성된 인덱스에 REUC 확장자를 기록하지 않습니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">표준 충돌된 병합 파일을 만듭니다.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">이름 변경에 대해 검토할 최대 유사성 소스(기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값을 초과하면 부정확한 이름 변경 감지가 중단됩니다. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의합니다.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">quiet</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션은 이 리베이스를 작업하는 다른 클라이언트가 조용한 리베이스 경험을 제공하도록 명령하며, 이는 애플리케이션별 방식으로 선택할 수 있습니다. 이는 libgit2에 직접적인 영향을 주지 않으며, Git 도구 간의 상호 운용성을 위해 제공됩니다.</p>
      </li>
      <li class="param-li">
        <span class="param-name">rewriteNotesRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description"><code>finish()</code>에서 사용되며, 리베이스가 완료될 때 리베이스된 커밋의 노트를 다시 작성하는 데 사용되는 노트 참조 이름입니다. NULL인 경우, 구성 옵션 <code>notes.rewriteRef</code>의 내용이 참고되며, <code>notes.rewrite.rebase</code> 구성 옵션이 false로 설정되지 않는 한 참고됩니다. <code>notes.rewriteRef</code>가 또한 NULL이면, 노트는 다시 작성되지 않습니다.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Rebase</span>
    <br>
    <p class="param-description">단계를 반복하고 적용하기 위한 초기화된 리베이스 핸들입니다.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">기존 리베이스를 찾지 못한 경우 예외를 발생시킵니다.</p>
  </li>
</ul>