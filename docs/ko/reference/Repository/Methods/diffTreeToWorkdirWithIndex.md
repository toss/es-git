# diffTreeToWorkdirWithIndex

트리(tree)와 작업 디렉터리(working directory) 간의 차이를 비교(diff)하면서 인덱스(index) 데이터를 반영해요.

이 메서드는 `git diff <tree>` 명령어를 모방해 동작해요. 즉,
1. 트리와 인덱스 간의 diff
2. 인덱스와 작업 디렉터리 간의 diff를 수행한 후, 두 결과를 합쳐 하나의 diff로 반환해요.

이를 통해 스테이징된(staged) 삭제나 추적된(tracked) 파일 변경 사항도 포함돼요.

## 시그니처

```ts
class Repository {
  diffTreeToWorkdirWithIndex(oldTree?: Tree, options?: DiffOptions): Diff;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oldTree</span><span class="param-type">null | Tree</span>
    <br>
    <p class="param-description">변경 전 파일("oldFile") 측에 사용할 트리예요. 전달하지 않으면 빈 트리가 사용돼요.</p>
  </li>
  <li class="param-li param-li-root">
  <span class="param-name">options</span><span class="param-type">null | DiffOptions</span>
  <br>
  <p class="param-description">diff 실행 방식을 설정하는 옵션이에요.</p>
  <ul class="param-ul">
    <li class="param-li">
      <span class="param-name">contextLines</span><span class="param-type">number</span>
      <br>
      <p class="param-description">
        변경된 부분 주변에서 표시할 동일한 코드 줄 개수를 설정해요. 기본값은 3이에요.
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        패스스펙(pathspec)을 사용할 때, 기본적으로 와일드카드(`fnmatch`)가 적용돼요.  
        이 옵션을 활성화하면 정확한 문자열 비교로 동작해요.
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">enableFastUntrackedDirs</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        기본적으로 Git은 추적되지 않은 디렉터리를 찾으면 내부 파일을 검사해요.  
        이 옵션을 활성화하면 이러한 검사를 생략하고, 디렉터리를 바로 "추적되지 않음" 상태로 설정해요.
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">forceBinary</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        모든 파일을 바이너리 파일로 간주하고 텍스트 diff를 비활성화해요.
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">forceText</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        모든 파일을 텍스트 파일로 간주하고 바이너리 속성 감지를 비활성화해요.
      </p>
    </li>
<li class="param-li">
  <span class="param-name">idAbbrev</span><span class="param-type">number</span>
  <br>
  <p class="param-description">
    OID(Object ID) 축약 길이를 설정해요.  
    기본값은 <code>core.abbrev</code> 설정값이며, 설정되지 않았을 경우 7이에요.
  </p>
</li>
    <li class="param-li">
      <span class="param-name">ignoreBlankLines</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">빈 줄을 무시하고 diff를 수행할지 설정해요.</p>
    </li>
    <li class="param-li">
      <span class="param-name">ignoreCase</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">대소문자를 구분하지 않고 diff를 수행할지 설정해요.</p>
    </li>
<li class="param-li">
  <span class="param-name">ignoreFilemode</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    파일 권한(mode) 변경을 무시할지 설정해요.  
    활성화하면 실행 권한 변경과 같은 파일 모드 변경 사항이 diff에서 제외돼요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">ignoreSubmodules</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    모든 서브모듈을 변경되지 않은 상태로 간주할지 설정해요.  
    활성화하면 diff에서 서브모듈 변경 사항을 무시해요.
  </p>
</li>
    <li class="param-li">
      <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">모든 공백을 무시하고 diff를 수행할지 설정해요.</p>
    </li>
    <li class="param-li">
      <span class="param-name">ignoreWhitespaceChange</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        공백 개수 변경을 무시할지 설정해요.  
        (예: 공백 1개 → 2개 변경은 무시되지만, 문자 변경은 감지됨)
      </p>
    </li>
    <li class="param-li">
      <span class="param-name">ignoreWhitespaceEol</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">줄 끝의 공백을 무시할지 설정해요.</p>
    </li>
<li class="param-li">
  <span class="param-name">includeIgnored</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    Git에서 무시(ignore)된 파일을 diff에 포함할지 설정해요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">includeTypechange</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    파일 타입이 변경된(typechange) 항목을 diff에 포함할지 설정해요.  
    활성화하면, 예를 들어 파일에서 디렉터리로 변경된 경우 이를 감지할 수 있어요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">includeTypechangeTrees</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    <code>includeTypechange</code>가 활성화된 경우,  
    기본적으로 삭제된 블롭(blob)으로 표시되던 항목을  
    올바르게 타입 변경(typechange) 기록으로 처리해요.  
    이때, <code>newFile</code>의 모드는 트리(tree)로 설정되지만,  
    트리의 SHA 값은 제공되지 않아요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">includeUnmodified</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    변경되지 않은 파일도 diff에 포함할지 설정해요.  
    기본적으로 변경되지 않은 파일은 diff에서 제외돼요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">includeUnreadable</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    읽을 수 없는(unreadable) 파일을 diff에 포함할지 설정해요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">includeUnreadableAsUntracked</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    읽을 수 없는(unreadable) 파일을 diff에서 추적되지 않은(untracked) 파일로 간주할지 설정해요.
  </p>
</li>
    <li class="param-li">
      <span class="param-name">includeUntracked</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        추적되지 않은(untracked) 파일을 diff에 포함할지 설정해요.
      </p>
    </li>
<li class="param-li">
  <span class="param-name">indentHeuristic</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    들여쓰기와 공백을 고려하는 휴리스틱(heuristic) 알고리즘을 사용해요.  
    이를 활성화하면 애매한 diff 구간(hunk)에서도 더 나은 비교 결과를 얻을 수 있어요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">interhunkLines</span><span class="param-type">number</span>
  <br>
  <p class="param-description">
    두 개의 diff 구간(hunk) 사이에 변경되지 않은 줄이  
    이 값보다 많으면 hunk를 하나로 병합해요. 기본값은 0이에요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">maxSize</span><span class="param-type">number</span>
  <br>
  <p class="param-description">
    이 크기(바이트 단위)보다 큰 블롭(blob)은 자동으로 바이너리로 처리돼요.  
    음수 값을 설정하면 이 기능을 비활성화할 수 있어요. 기본값은 512MB예요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">minimal</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    최소한의 diff를 찾기 위해 추가 연산을 수행할지 설정해요.  
    활성화하면 더 정확한 diff 결과를 얻을 수 있지만, 속도가 느려질 수 있어요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">newPrefix</span><span class="param-type">string</span>
  <br>
  <p class="param-description">
    새로운 파일 이름 앞에 붙일 가상 디렉터리 이름을 설정해요.  
    기본값은 <code>"b"</code>예요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">oldPrefix</span><span class="param-type">string</span>
  <br>
  <p class="param-description">
    이전 파일 이름 앞에 붙일 가상 디렉터리 이름을 설정해요.  
    기본값은 <code>"a"</code>예요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">pathspecs</span><span class="param-type">string[]</span>
  <br>
  <p class="param-description">
    diff를 적용할 파일 경로나 <code>fnmatch</code> 패턴을 설정해요.  
    지정한 패턴과 일치하는 파일만 diff에 포함돼요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">patience</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    "patience diff" 알고리즘을 사용할지 설정해요.  
    이 알고리즘은 변경된 줄을 보다 정확하게 매칭하지만, 성능이 느릴 수 있어요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">recurseIgnoredDirs</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    무시(ignore)된 디렉터리를 재귀적으로 탐색할지 설정해요.  
    활성화하면 내부의 파일까지 확인해요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">recurseUntrackedDirs</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    추적되지 않은(untracked) 디렉터리를 재귀적으로 탐색할지 설정해요.  
    활성화하면 내부의 파일까지 포함해 diff를 수행해요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">reverse</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    diff의 비교 방향을 반대로 할지 설정해요.  
    기본적으로 "old → new" 비교가 수행되지만,  
    이 옵션을 활성화하면 "new → old" 순서로 비교돼요.
  </p>
</li>
    <li class="param-li">
      <span class="param-name">showBinary</span><span class="param-type">boolean</span>
      <br>
      <p class="param-description">
        diff 결과에 바이너리 파일 정보를 포함할지 설정해요.
      </p>
    </li>
<li class="param-li">
  <span class="param-name">showUnmodified</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    diff 결과를 생성할 때, 변경되지 않은 파일의 이름을 포함할지 설정해요.  
    일반적으로 파일 목록을 출력하는 형식(name-only, name-status, raw)에서는  
    변경되지 않은 파일이 생략돼요.  
    하지만 패치 형식에서는 이 옵션을 활성화해도 포함되지 않아요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">showUntrackedContent</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    패치 형식의 diff를 생성할 때, 추적되지 않은(untracked) 파일의 내용을 포함할지 설정해요.  
    이 옵션을 활성화하면 <code>includeUntracked</code>도 자동으로 활성화되지만,  
    <code>recurseUntrackedDirs</code>는 활성화되지 않아요.  
    모든 추적되지 않은 파일의 내용을 포함하려면 <code>recurseUntrackedDirs</code>도 추가해야 해요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">skipBinaryCheck</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    diff에서 <code>binary</code> 플래그를 업데이트하지 않도록 설정해요.  
    이 옵션을 사용하면 diff를 반복적으로 순회할 때  
    파일을 완전히 로드하지 않고도 hunk와 데이터 콜백을 건너뛸 수 있어요.
  </p>
</li>
<li class="param-li">
  <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
  <br>
  <p class="param-description">
    작업 디렉터리의 파일이 인덱스와 다르게 변경되었지만,  
    OID(Object ID)가 동일한 경우 인덱스의 stat 정보를 업데이트할지 설정해요.  
    이 옵션을 활성화하지 않으면 diff는 인덱스를 수정하지 않아요.
  </p>
</li>
  </ul>
</li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Diff</span>
    <br>
    <p class="param-description">인덱스 데이터를 반영해 트리와 작업 디렉터리 간의 diff를 생성해요. 이를 통해 스테이징된 삭제, 추적된 파일 등의 변경 사항을 포함할 수 있어요.</p>
  </li>
</ul>