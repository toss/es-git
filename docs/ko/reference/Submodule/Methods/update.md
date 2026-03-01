# update

서브모듈을 업데이트해요.

이 작업은 누락된 서브모듈을 클론하고, 포함하고 있는 리포지토리의 인덱스에 지정된 커밋으로 서브리포지토리를 체크아웃해요. 서브모듈 리포지토리에 대상 커밋이 포함되어 있지 않으면, options에 제공된 fetch 옵션을 사용해 서브모듈을 fetch해요.

## 시그니처

```ts
class Submodule {
  update(
    init?: boolean | null | undefined,
    options?: SubmoduleUpdateOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">init</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">서브모듈이 아직 초기화되지 않았으면 먼저 초기화해야 하는지 여부</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | SubmoduleUpdateOptions</span>
    <br>
    <p class="param-description">업데이트를 위한 구성 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowFetch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">대상 커밋을 찾지 못했을 때 서브모듈의 기본 원격에서 fetch하는 것을 허용할지 여부. 기본값: <code>true</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkout</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description">이 옵션들은 checkout 단계에 전달되는 옵션</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 충돌이 있더라도 checkout을 취소하는 대신 안전한 파일 업데이트를 적용할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 조상(ancestor) 측의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌에 대해 일반 merge 파일을 작성할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 디렉터리를 생성할 때 사용할 모드 설정. 기본값은 0755예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">CRLF 변환 같은 필터를 적용할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description"><code>path</code>에 지정된 경로를 pathspec이 아니라 정확한 파일 경로로 처리할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌을 확인하되 실제 변경은 하지 않는 드라이 런으로 checkout을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">새 파일을 생성할 때 사용할 모드 설정. 기본값은 blob에 따라 0644 또는 0755예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">수정된 파일을 버릴 수도 있도록 작업 디렉터리를 대상과 일치시키기 위해 필요한 모든 작업을 수행할지 여부</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 우리(ours) 측의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">checkout 중에 무시된 파일을 덮어쓸지 여부. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 경로 추가. <code>disablePathspecMatch</code>가 설정되지 않으면 경로는 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로를 지정하지 않으면 모든 파일을 체크아웃해요. 그렇지 않으면 지정된 경로들만 체크아웃해요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업을 수행하기 전에 디스크에서 인덱스와 git 속성을 새로 고칠지 여부. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 무시된 파일을 제거할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">작업 디렉터리에서 추적되지 않는 파일을 제거할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">새 파일 생성은 허용하되 기존 파일이나 변경 사항은 덮어쓰지 않는 안전한 방식으로 checkout을 수행할지 여부. 이것이 기본값이에요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">인덱스 항목이 병합되지 않은 파일을 건너뛸지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">체크아웃할 디렉터리 설정</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">충돌에서 공통 그들(theirs) 측의 이름</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">checkout이 업데이트된 파일 정보을 인덱스에 기록하지 못하게 할지 여부. 기본값은 true예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">이미 존재하는 파일의 내용만 업데이트할지 여부. 설정하면 파일이 생성되거나 삭제되지 않아요. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 stage 2 버전(&quot;ours&quot;)을 사용해 checkout을 진행할지 여부. 기본값은 false예요.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">충돌 시 파일의 stage 3 버전(&quot;theirs&quot;)을 사용해 checkout을 진행할지 여부. 기본값은 false예요.</p>
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
            <p class="param-description">이 fetch 작업에 대한 추가 헤더 설정</p>
          </li>
          <li class="param-li">
            <span class="param-name">depth</span><span class="param-type">number</span>
            <br>
            <p class="param-description">fetch 깊이 설정. 0 이하의 값은 모든 것을 가져오라는 의미로 해석돼요(사실상 깊이 제한을 선언하지 않는 것과 동일해요).</p>
          </li>
          <li class="param-li">
            <span class="param-name">downloadTags</span><span class="param-type">AutotagOption</span>
            <br>
            <p class="param-description">원격의 태그에 대해 어떻게 동작할지 설정해요. 예를 들어 다운로드 중인 개체에 대한 태그를 자동으로 다운로드할지, 또는 전부 다운로드할지 등을 정해요. 기본값은 태그를 자동으로 따라가는 거예요.</p>
            <p class="param-description">- <code>Unspecified</code> : 원격 구성의 설정을 사용<br>- <code>Auto</code> : 이미 다운로드 중인 개체를 가리키는 태그를 서버에 요청<br>- <code>None</code> : refspecs를 넘는 어떤 태그도 요청하지 않음<br>- <code>All</code> : 모든 태그를 요청</p>
          </li>
          <li class="param-li">
            <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
            <br>
            <p class="param-description">원격 리다이렉션 설정. 다른 호스트로의 리다이렉트를 허용할지 여부예요. 기본적으로 git은 초기 요청(<code>/info/refs</code>)에서는 리다이렉트를 따르지만, 이후 요청에서는 따르지 않아요.</p>
            <p class="param-description">- <code>None</code> : fetch 또는 push의 어떤 단계에서도 사이트 외부 리다이렉트를 따르지 않음<br>- <code>Initial</code> : 초기 요청에서만 사이트 외부 리다이렉트를 허용. 이것이 기본값이에요.<br>- <code>All</code> : fetch 또는 push의 어떤 단계에서도 리다이렉트를 허용</p>
          </li>
          <li class="param-li">
            <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
            <br>
            <p class="param-description">fetch 작업에 사용할 프록시 옵션 설정</p>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">auto</span><span class="param-type">boolean</span>
                <br>
                <p class="param-description">git 구성에서 프록시를 자동 감지해 보세요. 이 설정은 이전에 지정한 <code>url</code>을 덮어써요.</p>
              </li>
              <li class="param-li">
                <span class="param-name">url</span><span class="param-type">string</span>
                <br>
                <p class="param-description">사용할 프록시의 정확한 URL을 지정해요. 이 설정은 이전에 지정한 <code>auto</code>를 덮어써요.</p>
              </li>
            </ul>
          </li>
          <li class="param-li">
            <span class="param-name">prune</span><span class="param-type">FetchPrune</span>
            <br>
            <p class="param-description">fetch 후 prune을 수행할지 여부 설정</p>
            <p class="param-description">- <code>Unspecified</code> : 구성의 설정을 사용.<br>- <code>On</code> : prune을 강제로 켬.<br>- <code>Off</code> : prune을 강제로 끔</p>
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