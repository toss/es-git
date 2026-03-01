# cherrypick

지정된 커밋을 HEAD에 체리픽하고 워킹 트리와 인덱스를 업데이트해요.
이 메서드는 커밋이 적용된 것처럼 인덱스와 트리를 준비하지만, 실제로 새 커밋을 만들지는 않아요.

## 시그니처

```ts
class Repository {
  cherrypick(
    commit: Commit,
    options?: CherrypickOptions | undefined | null,
  ): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">체리픽할 커밋</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CherrypickOptions</span>
    <br>
    <p class="param-description">체리픽 작업을 위한 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutOptions</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description">워킹 디렉터리를 업데이트할 때의 체크아웃 동작을 위한 옵션</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 충돌이 있더라도 체크아웃을 취소하는 대신 안전한 파일 업데이트를 적용할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 조상(ancestor) 쪽의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 일반 병합(merge) 파일을 작성할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리를 만들 때 사용할 모드. 기본값은 0755</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환 같은 필터를 적용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에 지정된 경로를 pathspec이 아닌 정확한 파일 경로로 취급할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌을 확인하되 실제 변경은 하지 않는 드라이 런을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일을 만들 때 사용할 모드. 기본값은 blob에 따라 0644 또는 0755</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">수정된 파일을 폐기할 수도 있도록 워킹 디렉터리를 대상과 일치시키는 데 필요한 모든 동작을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 our 쪽의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃 중 무시된 파일을 덮어쓸지 여부. 기본값은 true</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 경로를 추가해요. <code>disablePathspecMatch</code>가 설정되지 않았다면 이 경로는 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로를 지정하지 않으면 모든 파일을 체크아웃해요. 그렇지 않으면 지정된 경로만 체크아웃해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 전에 디스크에서 인덱스와 git attributes를 새로 고칠지 여부. 기본값은 true</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">워킹 디렉터리에서 무시된 파일을 제거할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">워킹 디렉터리에서 추적되지 않은 파일을 제거할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">기존 파일이나 변경 사항을 덮어쓰지 않으면서 새 파일 생성은 허용하는 안전한 방식으로 체크아웃을 수행할지 여부. 기본값</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">인덱스에 병합되지 않은 항목이 있는 파일을 건너뛸지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 디렉터리</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 their 쪽의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃이 업데이트된 파일 정보을 인덱스에 기록하지 못하게 할지 여부. 기본값은 true</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트할지 여부. 설정하면 파일을 생성하거나 삭제하지 않아요. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 stage 2 버전(&quot;ours&quot;)을 사용해서 체크아웃을 진행할지 여부. 기본값은 false</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 stage 3 버전(&quot;theirs&quot;)을 사용해서 체크아웃을 진행할지 여부. 기본값은 false</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">mainline</span><span class="param-type">number</span>
        <br>
        <p class="param-description">병합 커밋에 대한 부모 번호(1부터 시작). 병합 커밋을 체리픽할 때 mainline 부모는 체리픽을 가져올 대상으로 선택하는 부모예요. mainline은 병합이 수행된 기준 브랜치</p>
      </li>
      <li class="param-li">
        <span class="param-name">mergeOptions</span><span class="param-type">MergeOptions</span>
        <br>
        <p class="param-description">병합 커밋을 체리픽할 때의 병합 해결을 위한 옵션</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">diff3 스타일 파일을 생성할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌이 발생하면 충돌 해결을 계속 시도하지 않고 즉시 종료할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
            <br>
            <p class="param-description">충돌 해결에서 우선할 쪽</p>
          </li>
          <li class="param-li">
            <span class="param-name">findRenames</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">파일 이름 변경을 감지할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">모든 공백을 무시할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">공백 양의 변경을 무시할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">줄 끝의 공백을 무시할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">minimal</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">최소 diff를 찾기 위해 추가 시간을 사용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되는 커밋에 여러 병합 베이스가 있는 경우, 여러 병합 베이스를 병합해 재귀 병합 베이스를 만들지 않고 첫 번째 베이스만 사용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">patience</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">&quot;patience diff&quot; 알고리즘을 사용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">recursionLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">크로스 크로스 병합에서 가상 병합 베이스를 만들기 위해 공통 조상을 병합하는 최대 횟수. 이 제한에 도달하면 다음 조상은 병합을 시도하지 않고 그대로 사용해요. 기본값은 무제한</p>
          </li>
          <li class="param-li">
            <span class="param-name">renameThreshold</span><span class="param-type">number</span>
            <br>
            <p class="param-description">파일을 이름 변경으로 간주할 유사도(기본값 50)</p>
          </li>
          <li class="param-li">
            <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">단순화된 diff 파일을 위해 영숫자가 아닌 구간을 축약할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">생성된 인덱스에 REUC 확장을 기록하지 않을지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">표준 충돌 병합 파일을 생성할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">이름 변경을 검사할 유사도 소스의 최대 개수(기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값보다 크면 부정확한 이름 변경 감지를 중단해요. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의해요.</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">커밋이 병합 커밋인데 mainline이 지정되지 않은 경우.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">체리픽 작업 중 충돌이 발생한 경우.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const cherrypickCommit = repo.getCommit('cherrypick-commit');

// Cherrypick the commit onto HEAD and working tree
repo.cherrypick(cherrypickCommit);
repo.cleanupState();

// Cherrypick the commit against our commit selecting the first parent as mainline (This is necessary because, for merge commits, there is ambiguity about which side of the merge should be treated as the baseline.)
repo.cherrypick(cherrypickCommit, { mainline: 1 });
repo.cleanupState();

// Prevent working tree changes (dry run) but compute conflicts
repo.cherrypick(cherrypickCommit, { checkoutOptions: { dryRun: true } });
repo.cleanupState();

// Cherrypick the commit against our commit selecting the first parent as mainline and prevent working tree changes (dry run) but compute conflicts
repo.cherrypick(cherrypickCommit, { mainline: 1, checkoutOptions: { dryRun: true } });
repo.cleanupState();
```