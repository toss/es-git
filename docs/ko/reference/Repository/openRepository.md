# openRepository

주어진 경로에서 기존 리포지토리를 열어요.

## 시그니처

```ts
function openRepository(
  path: string,
  options?: RepositoryOpenOptions,
  signal?: AbortSignal,
): Promise<Repository>;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">이미 존재하는 리포지토리가 위치한 디렉터리 경로예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryOpenOptions</span>
    <br>
    <p class="param-description">리포지토리를 오픈할 때 적용할 옵션들을 설정할 수 있어요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">bare</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션이 <code>true</code>로 설정되면, 리포지토리가 bare하지 않더라도 강제로 bare 형태로 열며, 워킹 디렉터리는 무시하고 리포지토리 구성을 성능 향상을 위해 지연해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ceilingDirs</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">부모 디렉터리 탐색 중에, 진입하기 전에 멈출 경로들의 목록을 지정해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">crossFs</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션이 <code>true</code>로 설정되면, 상위 디렉터리 탐색 시 파일시스템 경계를 넘지 않도록 해요 (stat의 st_dev 값이 변경될 때 감지해요).</p>
      </li>
      <li class="param-li">
        <span class="param-name">fromEnv</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션이 <code>true</code>로 설정되면, <code>open</code>은 다른 옵션과 <code>ceilingDirs</code>를 무시하고 git이 사용하는 환경 변수를 따르게 돼요. 다만, <code>path</code>가 <code>$GIT_DIR</code>을 덮어써요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noDotgit</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션이 <code>true</code>로 설정되면, <code>path</code>에 <code>/.git</code>을 덧붙이지 않아요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noSearch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">이 옵션이 <code>true</code>로 설정되면, 경로가 리포지토리를 직접 가리켜야 해요. 그렇지 않으면 <code>open</code>이 상위 디렉터리로 탐색해요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">작업을 취소할 수 있도록 <code>AbortSignal</code>을 설정할 수 있어요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">열린 리포지토리를 반환해요.</p>
  </li>
</ul>

## 예제

기본 예시

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
```

`bare` 리포지토리를 열어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo.git', {
  bare: true,
});
```