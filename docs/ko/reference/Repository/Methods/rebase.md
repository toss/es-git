# rebase

`branch`에서 `upstream`에 포함되지 않은 커밋들을 선택해 `onto`로 지정된 새 기준(base) 위에 재적용하는 리베이스 작업을 초기화합니다. 

반환된 `Rebase`를 순회(`for (const op of rebase) { ... }` 또는 `next()` 호출)하여 리베이스를 진행하세요.

## 시그니처

```ts
class Repository {
  rebase(
    branch?: AnnotatedCommit | undefined | null,
    upstream?: AnnotatedCommit | undefined | null,
    onto?: AnnotatedCommit | undefined | null,
    options?: RebaseOptions | undefined | null,
  ): Rebase;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">branch</span><span class="param-type">null | AnnotatedCommit</span>
    <br>
    <p class="param-description">리베이스할 브랜치를 나타내는 Annotated Commit이에요. 일반적으로 브랜치의 <code>HEAD</code> 커밋이에요. 생략하면 현재 체크아웃된 브랜치를 사용해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">upstream</span><span class="param-type">null | AnnotatedCommit</span>
    <br>
    <p class="param-description">리베이스할 커밋의 &quot;원래 기반&quot;을 정의하는 Annotated Commit이에요. 생략하면 일반적으로 리포지토리는 브랜치의 구성된 업스트림을 사용하려고 해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">onto</span><span class="param-type">null | AnnotatedCommit</span>
    <br>
    <p class="param-description">선택한 커밋이 적용될 &quot;새 기반&quot;을 지정해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RebaseOptions</span>
    <br>
    <p class="param-description">체크아웃 옵션, 병합 옵션 및 메모리 내 리베이스와 같은 리베이스 동작에 대한 세부적인 제어를 제공해요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkoutOptions</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description"><code>Repository::rebase</code>, <code>next()</code> 및 <code>abort()</code> 중에 파일이 작성되는 방식에 대한 옵션이에요. <code>init</code> 및 <code>next</code>에서는 최소 전략인 <code>GIT_CHECKOUT_SAFE</code>가 기본값이며, <code>abort</code>에서는 Git 의미론을 맞추기 위해 최소 전략 <code>GIT_CHECKOUT_FORCE</code>가 기본값이에요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 충돌이 발생하더라도 체크아웃을 취소하는 대신 안전한 파일 업데이트를 적용해요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 조상 쪽 이름이에요</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대한 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부를 나타내요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대한 일반 병합 파일을 작성할지 여부를 나타내요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리가 생성될 때 모드를 설정해요. 기본값은 0755에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환과 같은 필터를 적용할지 여부를 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에서 지정한 경로를 경로 명세가 아닌 정확한 파일 경로로 취급해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌을 확인하지만 실제로 변경을 가하지 않는 드라이런을 수행할지 여부를 나타내요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일이 생성될 때 모드를 설정해요. 기본값은 blob에 따라 0644 또는 0755에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리를 목표와 일치시키기 위해 수정된 파일을 무시하는 등의 모든 조치를 취해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 측면을 나타내는 우리 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃 중에 무시된 파일이 덮어쓰여질지 여부를 나타내요. 기본값은 true에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 경로를 추가해요. 경로는 <code>disablePathspecMatch</code>가 설정되지 않는 한 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로가 지정되지 않으면 모든 파일이 체크아웃되고, 그렇지 않으면 지정된 경로만 체크아웃돼요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성해요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 전에 인덱스와 Git 속성을 디스크에서 새로 고칠지 여부를 나타내요. 기본값은 true에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 무시된 파일을 제거해요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 추적되지 않는 파일을 제거해요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">새 파일을 생성할 수 있지만 기존 파일이나 변경 사항을 덮어쓰지 않는 안전한 방식으로 체크아웃이 수행되어야 함을 나타내요. 이것이 기본값이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되지 않은 인덱스 항목이 있는 파일을 건너떠요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 디렉터리를 설정해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 상대 측면을 나타내는 이름이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃이 업데이트된 파일의 정보를 인덱스에 쓰지 않도록 방지해요. 기본값은 true에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트해요. 설정하면 파일이 생성되거나 삭제되지 않아요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃이 충돌 시 파일의 스테이지 2 버전(&quot;ours&quot;)을 사용하도록 할지 여부를 나타내요. 기본값은 false에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">체크아웃이 충돌 시 파일의 스테이지 3 버전(&quot;theirs&quot;)을 사용하도록 할지 여부를 나타내요. 기본값은 false에요.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">inmemory</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이는 메모리 내 리베이스를 시작하여 호출자가 리베이스 작업을 단계적으로 진행하고 리베이스된 변경 사항을 커밋할 수 있게 하지만, HEAD를 전환하거나 리포지토리가 리베이스 상태로 업데이트되지 않도록 해요. 작업 디렉터리(있는 경우)와 간섭하지 않아요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mergeOptions</span><span class="param-type">MergeOptions</span>
        <br>
        <p class="param-description"><code>next()</code> 동안 트리를 병합하는 방식을 제어하는 옵션이에요.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">diff3Style</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">diff3 스타일 파일을 생성해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">failOnConflict</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌이 발생할 경우, 계속해서 충돌 해결을 시도하지 않고 즉시 종료해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filFavor</span><span class="param-type">FileFavor</span>
            <br>
            <p class="param-description">충돌 해결을 위해 선호할 측을 지정해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">findRenames</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">파일 이름 변경을 감지해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">모든 공백을 무시해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">공백의 양의 변경을 무시해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">행 끝의 공백을 무시해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">minimal</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">최소한의 차이를 찾기 위해 추가 시간을 사용해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">noRecursive</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되는 커밋이 여러 병합 기반을 가지고 있는 경우, 이를 병합하여 재귀적인 병합 기반을 생성하지 않고, 대신 단순히 첫 번째 기반을 사용해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">patience</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">&quot;인내심 있는 차이&quot; 알고리즘을 사용해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recursionLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">크리스크로스 병합을 만났을 때 가상 병합 기반을 구성하기 위해 공통 조상을 병합하는 최대 횟수에요. 이 제한에 도달하면 다음 조상은 병합 시도를 중단하고 단순히 사용돼요. 기본값은 무제한이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">renameThreshold</span><span class="param-type">number</span>
            <br>
            <p class="param-description">파일을 이름 변경으로 간주할 유사도 (기본값 50)이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">simplifyAlnum</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">간소화된 차이 파일을 위해 비 알파벳-숫자 영역을 축소해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipReuc</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">생성된 인덱스에 REUC 확장을 기록하지 않아요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">standardStyle</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">표준 충돌된 병합 파일을 생성해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetLimit</span><span class="param-type">number</span>
            <br>
            <p class="param-description">이름 변경에 대해 검사할 최대 유사도 소스 (기본 200) 이에요. 이름 변경 후보 (추가 / 삭제 쌍)의 수가 이 값을 초과하면 부정확한 이름 변경 감지가 중단돼요. 이 설정은 <code>merge.renameLimit</code> 구성 값을 재정의해요.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">quiet</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">다른 클라이언트에게 조용한 리베이스 경험을 제공하도록 지시하여 애플리케이션별 방식으로 이를 제공할 수 있어요. 이는 직접적으로 libgit2에 영향을 미치지 않지만 Git 도구 간의 상호 운용성을 위해 제공돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">rewriteNotesRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description"><code>finish()</code>에서 사용되며, 리베이스가 끝날 때 리베이스된 커밋의 노트를 재작성하는 데 사용되는 노트 참조의 이름이에요. NULL인 경우 구성 옵션 <code>notes.rewriteRef</code>의 내용이 검토돼요. 구성 옵션 <code>notes.rewrite.rebase</code>가 false로 설정된 경우를 제외하고요. <code>notes.rewriteRef</code>도 NULL인 경우 노트는 재작성되지 않아요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Rebase</span>
    <br>
    <p class="param-description">단계를 반복하고 적용하는 초기화된 리베이스 핸들이에요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const branchRef = repo.getReference('refs/heads/other');
const upstreamRef = repo.getReference('refs/heads/main');
const branch = repo.getAnnotatedCommitFromReference(branchRef);
const upstream = repo.getAnnotatedCommitFromReference(upstreamRef);

const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };

const rebase = repo.rebase(branch, upstream);
for (const op of rebase) {
  rebase.commit({ committer: sig });
}
rebase.finish(sig);
```