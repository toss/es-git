# initRepository

지정된 디렉터리에 새로운 리포지토리를 생성해요.

## 시그니처

```ts
function initRepository(
  path: string,
  options?: RepositoryInitOptions,
  signal?: AbortSignal,
): Promise<Repository>;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">새로운 리포지토리를 생성할 디렉터리 경로예요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RepositoryInitOptions</span>
    <br>
    <p class="param-description">리포지토리 초기화 방식을 설정하는 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">bare</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>bare</code> 리포지토리로 생성할지 설정해요. <code>true</code>로 설정하면 워킹 디렉터리 없이 <code>.git</code> 디렉터리만 생성돼요. 기본값은 <code>false</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">description</span><span class="param-type">string</span>
        <br>
         <p class="param-description">
          <code>.git/description</code> 파일의 내용을 설정해요.  
          설정하지 않으면 기본 템플릿 내용이 사용돼요.
        </p>
      </li>
      <li class="param-li">
        <span class="param-name">externalTemplate</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">외부 템플릿을 사용할지 설정해요. 활성화하면 <code>templatePath</code>를 먼저 확인한 후, <code>git config init.templatedir</code>, <code>/usr/share/git-core-templates</code>(존재하는 경우) 순서로 검색해요. 기본값은 <code>true</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">initialHead</span><span class="param-type">string</span>
        <br>
        <p class="param-description"><code>HEAD</code>가 가리킬 브랜치 이름을 설정해요. 설정하지 않으면 Git의 기본 설정을 따르고, <code>refs/</code>로 시작하지 않으면 자동으로 <code>refs/heads/</code>가 추가돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mkdir</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">필요한 경우 리포지토리 경로를 자동으로 생성할지 설정해요. 기본값은 <code>true</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mkpath</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">리포지토리 및 워킹 디렉터리 경로를 자동으로 생성할지 설정해요. 기본값은 <code>true</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-type">number</span>
        <br>
        <p class="param-description"><code>RepositoryInit</code> 상수 중 하나를 설정하거나, 사용자 지정 값을 입력할 수 있어요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noDotgitDir</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">기본적으로 <code>.git/</code> 디렉터리가 리포지토리 경로에 자동으로 추가되지만, 이 옵션을 설정하면 해당 동작을 방지할 수 있어요. 기본값은 <code>false</code>예요.<code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">noReinit</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">해당 경로에 이미 리포지토리가 존재하는 경우 오류를 발생시킬지 설정해요. 기본값은 <code>false</code>예요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">originUrl</span><span class="param-type">string</span>
        <br>
        <p class="param-description"> 설정하면 <code>origin</code> 리모트가 자동으로 추가돼요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">templatePath</span><span class="param-type">string</span>
        <br>
        <p class="param-description"><code>externalTemplate</code> 옵션이 활성화된 경우, 사용할 템플릿 디렉터리 경로를 설정해요. 설정하지 않으면 기본 위치에서 템플릿을 검색해요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">workdirPath</span><span class="param-type">string</span>
        <br>
        <p class="param-description">워킹 디렉터리의 경로를 설정해요. 상대 경로를 입력하면 리포지토리 경로를 기준으로 설정돼요. <code>bare</code> 리포지토리가 아닌 경우 <code>.git</code> 내부에 <code>gitlink</code> 파일이 생성돼요.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">초기화 작업을 취소할 수 있도록 <code>AbortSignal</code>을 설정할 수 있어요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description"> 생성된 리포지토리를 반환해요.</p>
  </li>
</ul>

## 예제

기본 예시예요.

```ts
import { initRepository } from 'es-git';

const repo = await iniRepository('/path/to/repo');
```

`bare` 리포지토리를 생성해요.

```ts
import { initRepository } from 'es-git';

const repo = await iniRepository('/path/to/repo.git', {
  bare: true,
});
```