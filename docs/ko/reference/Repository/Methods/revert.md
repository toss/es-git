# revert

지정된 커밋을 되돌리고, 해당 변경 사항의 역을 HEAD 커밋과 작업 디렉터리에 적용해요.

## 시그니처

```ts
class Repository {
  revert(
    commit: Commit,
    options?: RevertOptions | undefined | null,
  ): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">되돌릴 커밋.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">RevertOptions | null</span>
    <br>
    <p class="param-description">되돌리기 작업의 옵션.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutOptions</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description">작업 디렉터리를 업데이트할 때의 체크아웃 동작 옵션.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 충돌이 있을 때 체크아웃을 취소하는 대신 안전한 파일 업데이트를 적용해요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 조상 측 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부를 나타내요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 일반 병합 파일을 작성할지 여부를 나타내요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리를 생성할 때 사용할 모드를 설정해요. 기본값은 0755예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환과 같은 필터를 적용할지 여부를 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에 지정된 경로를 pathspec이 아닌 정확한 파일 경로로 처리해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">실제 변경 없이 충돌 확인만 수행하는 드라이 런으로 체크아웃을 실행할지 여부를 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일을 생성할 때 사용할 모드를 설정해요. 기본값은 blob에 따라 0644 또는 0755예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">수정된 파일을 삭제하는 것을 포함하여 작업 디렉터리를 대상과 일치시키기 위해 필요한 모든 작업을 수행해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 our 측 공통 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃 중 무시된 파일을 덮어쓸지 여부를 나타내요. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 경로를 추가해요. <code>disablePathspecMatch</code>가 설정되지 않은 경우 해당 경로는 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로가 지정되지 않으면 모든 파일이 체크아웃돼요. 그렇지 않으면 지정된 경로만 체크아웃돼요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성해요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 전에 디스크에서 인덱스와 git 속성을 새로 고칠지 여부를 나타내요. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 무시된 파일을 제거해요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 추적되지 않는 파일을 제거해요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">기존 파일이나 변경 사항을 덮어쓰지 않고 새 파일 생성을 허용하는 안전한 방식으로 체크아웃을 수행할지 여부를 나타내요. 이것이 기본값이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되지 않은 인덱스 항목이 있는 파일을 건너뛰어요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 디렉터리 설정.</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 their 측 공통 이름.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">업데이트된 파일 정보를 인덱스에 쓰는 것을 방지해요. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트해요. 설정된 경우 파일이 생성되거나 삭제되지 않아요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 스테이지 2 버전(&quot;ours&quot;)을 사용하여 체크아웃을 진행할지 여부를 나타내요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 스테이지 3 버전(&quot;theirs&quot;)을 사용하여 체크아웃을 진행할지 여부를 나타내요. 기본값은 false예요.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">mainline</span><span class="param-type">number</span>
        <br>
        <p class="param-description">병합 커밋의 부모 번호(1부터 시작). 병합 커밋을 되돌릴 때, mainline 부모는 되돌리려는 대상이에요. mainline은 병합이 이루어진 브랜치예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mergeOptions</span><span class="param-type">MergeOptions</span>
        <br>
        <p class="param-description">병합 충돌 해결 옵션.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">diff3 스타일 파일 생성.</p>
          </li>
          <li class="param-li">
            <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌이 발생하면 충돌 해결을 계속 시도하지 않고 즉시 종료해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
            <br>
            <p class="param-description">충돌 해결 시 우선할 측 지정.</p>
          </li>
          <li class="param-li">
            <span class="param-name">findRenames</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">파일 이름 변경 감지.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">모든 공백 무시.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">공백 양의 변경 무시.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">줄 끝의 공백 무시.</p>
          </li>
          <li class="param-li">
            <span class="param-name">minimal</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">최소 diff를 찾기 위해 추가 시간 사용.</p>
          </li>
          <li class="param-li">
            <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되는 커밋에 여러 병합 베이스가 있는 경우, 여러 병합 베이스를 병합하여 재귀적 병합 베이스를 구축하는 대신 단순히 첫 번째 베이스를 사용해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">patience</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">&quot;patience diff&quot; 알고리즘 사용.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recursionLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">교차 병합 시 가상 병합 베이스를 구축하기 위해 공통 조상을 병합하는 최대 횟수. 이 한도에 도달하면 다음 조상을 병합 시도 없이 그대로 사용해요. 기본값은 무제한이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">renameThreshold</span><span class="param-type">number</span>
            <br>
            <p class="param-description">파일 이름 변경으로 간주할 유사도(기본값 50).</p>
          </li>
          <li class="param-li">
            <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">단순화된 diff 파일을 위해 영숫자가 아닌 영역 압축.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">생성된 인덱스에 REUC 확장을 작성하지 않아요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">표준 충돌 병합 파일 생성.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">이름 변경 감지를 위해 검사할 최대 유사도 소스 수(기본값 200). 이름 변경 후보(추가/삭제 쌍)의 수가 이 값보다 크면 부정확한 이름 변경 감지가 중단돼요. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의해요.</p>
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
    <p class="param-description">커밋이 병합 커밋이고 mainline이 지정되지 않은 경우.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">되돌리기 작업 중 충돌이 발생한 경우.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const last = repo.head().target()!;
const commit = repo.getCommit(last);

// Revert and update working tree
repo.revert(commit);
repo.cleanupState();

// Revert a merge commit: specify the mainline parent
// repo.revert(mergeCommit, { mainline: 1 });
// repo.cleanupState();
```