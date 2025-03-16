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
    <p class="param-description">열려고 하는 기존 리포지토리의 디렉터리 경로예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryOpenOptions</span>
    <br>
    <p class="param-description">리포지토리 여는 방식을 설정하는 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">ceilingDirs</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">상위 디렉터리를 검색할 때, 이 목록에 포함된 디렉터리 이전에서 검색을 멈춰요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">flags</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">
<code>flags</code>에 따라 리포지토리를 여는 방식이 달라져요.
<br>
- <code>RepositoryOpenFlags.NoSearch</code> : 
  <code>path</code>가 반드시 리포지토리 경로여야 해요.  
  (<code>open</code>이 상위 디렉터리를 검색하지 않음)  
  설정하지 않으면 서브디렉터리에서 호출해도 상위 디렉터리를 검색해요.
<br>
- <code>RepositoryOpenFlags.CrossFS</code> :
  상위 디렉터리를 검색할 때 파일 시스템 경계를 넘지 않도록 설정해요.  
  (<code>stat</code>의 <code>st_dev</code> 필드가 변경되면 검색을 중단)
<br>
- <code>RepositoryOpenFlags.Bare</code> :
  <code>bare</code> 리포지토리로 강제로 열어요.  
  (워킹 디렉터리를 무시하고, 성능을 위해 리포지토리 설정을 지연 로드)
<br>
- <code>RepositoryOpenFlags.NoDotgit</code> :
  <code>path</code>에 자동으로 <code>/.git</code>을 추가하는 동작을 방지해요.
<br>
- <code>RepositoryOpenFlags.FromEnv</code> :
  환경 변수를 기반으로 리포지토리를 열어요.  
  (다른 플래그 및 <code>ceilingDirs</code> 설정을 무시하고 Git의 환경 변수 사용)  
  단, <code>path</code> 값이 <code>$GIT_DIR</code>보다 우선돼요.</p>
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

bare 리포지토리를 열어요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo.git', {
  bare: true,
});
```

하위 경로에서 리포지토리를 열어요.

```ts
import { openRepository, RepositoryOpenFlags } from 'es-git';

const repo = await openRepository('/path/to/repo/sub/dir', {
  flags: RepositoryOpenFlags.CrossFS,
});
```