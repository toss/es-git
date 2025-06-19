# checkoutTree

treeish가 가리키는 트리의 내용과 일치하도록 인덱스와 작업 트리의 파일을 업데이트해요.

## 시그니처

```ts
class Repository {
  checkoutTree(treeish: GitObject, options?: CheckoutOptions | undefined | null): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">treeish</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">GitObject</span>
    <br>
    <p class="param-description">트리를 가리키는 Git 개체</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CheckoutOptions</span>
    <br>
    <p class="param-description">체크아웃 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">안전 모드에서 체크아웃을 취소하는 대신 충돌이 있어도 안전한 파일 업데이트를 적용해요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 공통 조상 측 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌에 대한 diff3 형식 파일에 공통 조상 데이터를 포함할지 여부를 나타내요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">충돌에 대해 일반 병합 파일을 작성해야 하는지 여부를 나타내요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">dirPerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">새 디렉토리가 생성되는 모드를 설정해요. 기본값은 0755예요.</p>
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
        <p class="param-description">이 체크아웃이 충돌을 확인하지만 실제 변경 사항은 만들지 않는 드라이런을 수행해야 함을 나타내요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">filePerm</span><span class="param-type">number</span>
        <br>
        <p class="param-description">새 파일이 생성되는 모드를 설정해요. 기본값은 blob에서 지정한 대로 0644 또는 0755예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">수정된 파일을 잠재적으로 삭제하는 것을 포함하여 작업 디렉토리를 대상과 일치시키는 데 필요한 모든 조치를 취해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ourLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 우리 측 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃 중에 무시된 파일을 덮어써야 하는지 여부를 나타내요. 기본값은 true예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">체크아웃할 경로를 추가해요. 경로는 <code>disablePathspecMatch</code>가 설정되지 않는 한 &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; 패턴이에요. 경로가 지정되지 않으면 모든 파일이 체크아웃돼요. 그렇지 않으면 지정된 경로만 체크아웃돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">안전 모드에서 존재하지 않는 파일을 생성해요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">refresh</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 전에 인덱스와 git 속성을 디스크에서 새로고침해야 하는지 여부를 나타내요. 기본값은 true예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉토리에서 무시된 파일을 제거해요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">작업 디렉토리에서 추적되지 않은 파일을 제거해요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">safe</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 안전하게 수행되어야 함을 나타내요. 새 파일 생성은 허용하지만 기존 파일이나 변경 사항은 덮어쓰지 않아요. 이것이 기본값이에요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">병합되지 않은 인덱스 항목이 있는 파일을 건너뛰어요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">targetDir</span><span class="param-type">string</span>
        <br>
        <p class="param-description">체크아웃할 디렉토리 설정</p>
      </li>
      <li class="param-li">
        <span class="param-name">theirLabel</span><span class="param-type">string</span>
        <br>
        <p class="param-description">충돌의 그들 측 이름</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 업데이트된 파일 정보를 인덱스에 쓰지 못하도록 해요. 기본값은 true예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이미 존재하는 파일의 내용만 업데이트해요. 설정하면 파일이 생성되거나 삭제되지 않아요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useOurs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 충돌 시 파일의 스테이지 2 버전(&quot;ours&quot;)을 사용하여 진행해야 하는지 여부를 나타내요. 기본값은 false예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">체크아웃이 충돌 시 파일의 스테이지 3 버전(&quot;theirs&quot;)을 사용하여 진행해야 하는지 여부를 나타내요. 기본값은 false예요.</p>
      </li>
    </ul>
  </li>
</ul>