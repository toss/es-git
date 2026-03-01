# clone

새로 생성된 서브모듈에 대해 클론 단계를 수행해요.

이 작업은 새로 생성된 서브모듈을 설정하기 위해 필요한 `git clone`을 수행해요.

## 시그니처

```ts
class Submodule {
  clone(
    options?: SubmoduleUpdateOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<Repository>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | SubmoduleUpdateOptions</span>
    <br>
    <p class="param-description">사용할 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowFetch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">대상 커밋을 찾을 수 없을 때 서브모듈의 기본 리모트에서 fetch를 허용할지 여부. 기본값: <code>true</code></p>
      </li>
      <li class="param-li">
        <span class="param-name">checkout</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description">이 옵션은 checkout 단계에 전달되는 옵션</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 충돌이 있더라도 checkout을 취소하는 대신 안전한 파일 업데이트를 적용할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 조상(ancestor) 측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 일반 merge 파일을 작성할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리를 생성할 때 사용할 모드. 기본값: 0755</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환 같은 필터를 적용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에 지정된 경로를 pathspec이 아니라 정확한 파일 경로로 취급할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 여부만 확인하고 실제 변경은 하지 않는 드라이 런으로 checkout을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일을 생성할 때 사용할 모드. 기본값은 blob에 따라 0644 또는 0755</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">수정된 파일을 폐기할 수 있는 경우를 포함해 작업 디렉터리가 대상과 일치하도록 필요한 모든 동작을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 ours 측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">checkout 중에 무시된 파일을 덮어쓸지 여부. 기본값: true</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">checkout할 경로 추가. <code>disablePathspecMatch</code>가 설정되지 않았다면 경로는 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로를 지정하지 않으면 모든 파일을 checkout해요. 그렇지 않으면 지정된 경로만 checkout해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업을 수행하기 전에 디스크에서 인덱스와 git 속성을 새로고침할지 여부. 기본값: true</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 무시된 파일을 제거할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 추적되지 않은 파일을 제거할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">새 파일은 생성할 수 있지만 기존 파일이나 변경 사항은 덮어쓰지 않도록 안전하게 checkout을 수행할지 여부. 이 값이 기본이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">병합되지 않은 인덱스 엔트리가 있는 파일을 건너뛸지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">checkout할 디렉터리</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌의 공통 theirs 측 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">checkout이 업데이트된 파일 정보를 인덱스에 기록하지 못하게 할지 여부. 기본값: true</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트할지 여부. 설정하면 파일을 생성하거나 삭제하지 않아요. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 스테이지 2 버전(&quot;ours&quot;)을 사용해 checkout을 진행할지 여부. 기본값: false</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 스테이지 3 버전(&quot;theirs&quot;)을 사용해 checkout을 진행할지 여부. 기본값: false</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">fetch</span><span class="param-type">FetchOptions</span>
        <br>
        <p class="param-description">콜백을 포함해 fetch를 제어하는 옵션</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">credential</span><span class="param-type">Credential</span>
            <br>
            <p class="param-description">libgit2에서 git 자격 증명을 나타내는 인터페이스</p>
          </li>
          <li class="param-li">
            <span class="param-name">customHeaders</span><span class="param-type">string[]</span>
            <br>
            <p class="param-description">이 fetch 작업에 추가 헤더 설정</p>
          </li>
          <li class="param-li">
            <span class="param-name">depth</span><span class="param-type">number</span>
            <br>
            <p class="param-description">fetch 깊이 설정. 0 이하 값은 모든 항목을 가져오라는 의미로 해석돼요(사실상 깊이 제한을 선언하지 않은 것과 같아요).</p>
          </li>
          <li class="param-li">
            <span class="param-name">downloadTags</span><span class="param-type">AutotagOption</span>
            <br>
            <p class="param-description">리모트의 태그에 대해 어떻게 동작할지 설정해요(예: 다운로드 중인 개체에 대한 태그를 자동으로 다운로드하거나, 모두 다운로드). 기본값은 태그 자동 추적이에요.</p>
            <p class="param-description">- <code>Unspecified</code> : 리모트 구성의 설정을 사용<br>- <code>Auto</code> : 이미 다운로드 중인 개체를 가리키는 태그를 서버에 요청<br>- <code>None</code> : refspec을 넘어서는 어떤 태그도 요청하지 않음<br>- <code>All</code> : 모든 태그를 요청</p>
          </li>
          <li class="param-li">
            <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
            <br>
            <p class="param-description">리모트 리디렉션 설정(다른 호스트로의 리디렉션 허용 여부) 설정. 기본적으로 git은 초기 요청(<code>/info/refs</code>)에서는 리디렉션을 따르지만 이후 요청에서는 따르지 않아요.</p>
            <p class="param-description">- <code>None</code> : fetch 또는 push의 어떤 단계에서도 오프사이트 리디렉션을 따르지 않음<br>- <code>Initial</code> : 초기 요청에서만 오프사이트 리디렉션을 허용. 기본값<br>- <code>All</code> : fetch 또는 push의 모든 단계에서 리디렉션을 허용</p>
          </li>
          <li class="param-li">
            <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
            <br>
            <p class="param-description">fetch 작업에 사용할 프록시 옵션 설정</p>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">auto</span><span class="param-type">boolean</span>
                <br>
                <p class="param-description">git 구성에서 프록시를 자동 감지하려고 시도해요. 이전에 지정된 <code>url</code>을 덮어써요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">url</span><span class="param-type">string</span>
                <br>
                <p class="param-description">사용할 프록시의 정확한 URL 지정. 이전에 지정된 <code>auto</code>를 덮어써요.</p>
              </li>
            </ul>
          </li>
          <li class="param-li">
            <span class="param-name">prune</span><span class="param-type">FetchPrune</span>
            <br>
            <p class="param-description">fetch 이후 prune 수행 여부 설정</p>
            <p class="param-description">- <code>Unspecified</code> : 구성의 설정을 사용<br>- <code>On</code> : prune을 강제로 켬<br>- <code>Off</code> : prune을 강제로 끔</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">작업을 취소하기 위한 선택적 AbortSignal</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">새로 생성된 리포지토리 개체</p>
  </li>
</ul>