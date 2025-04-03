# discoverRepository

주어진 경로 또는 상위 디렉터리에서 기존 리포지토리를 찾아 열어요.

이 함수는 `path`에서 시작해 파일 시스템을 위로 탐색하며 `.git` 디렉터리를 찾을 때까지 반복해요.

## 시그니처

```ts
function discoverRepository(path: string, signal?: AbortSignal | null | undefined): Promise<Repository>;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
      리포지토리를 검색할 디렉터리 경로예요.
    </p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">
      검색 작업을 취소할 수 있도록 `AbortSignal`을 설정할 수 있어요.
    </p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description"> 발견된 Git 리포지토리를 반환해요.</p>
  </li>
</ul>
