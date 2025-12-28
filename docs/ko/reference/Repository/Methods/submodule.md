# submodule

체크아웃을 위해 새 git 서브모듈을 설정해요.

이 기능은 서브모듈 내용의 fetch와 checkout까지 "git submodule add"를 수행해요. 새 서브모듈을 준비하고, `.gitmodules`에 항목을 만들고, 작업 디렉터리의 지정된 경로에 또는 `.git/modules`에 빈 초기화된 리포지토리를 만든 다음, 작업 디렉터리에서 새 리포지토리로 향하는 gitlink를 만들어요.

"git submodule add"를 완전히 에뮬레이트하려면 이 함수를 호출한 다음, 서브모듈 리포지토리를 `open()`하고 필요에 따라 clone 단계를 수행해요. 마지막으로 `addFinalize()`를 호출해 새 서브모듈과 `.gitmodules`를 인덱스에 추가하는 작업을 마무리해 커밋할 준비를 해요.

## 시그니처

```ts
class Repository {
  submodule(
    url: string,
    path: string,
    useGitlink?: boolean | null | undefined,
  ): Submodule;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서브모듈의 원격 URL</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서브모듈이 생성되어야 하는 경로</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">useGitlink</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">작업 디렉터리에 작업 디렉터리 내의 리포지토리 대신 <code>.git/modules</code>의 리포지토리를 가리키는 gitlink를 포함할지 여부</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Submodule</span>
    <br>
    <p class="param-description">서브모듈</p>
  </li>
</ul>