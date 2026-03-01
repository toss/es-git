# repoInit

클론을 준비하기 위해 서브모듈의 서브리포지토리를 설정해요.

이 함수는 원격에서 클론하기 전에, 서브모듈로부터 서브모듈 리포지토리를 초기화하고 설정하는 데 호출할 수 있어요.

## 시그니처

```ts
class Submodule {
  repoInit(
    useGitlink?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<Repository>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">useGitlink</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">workdir에 리포지토리를 직접 두는 대신, <code>.git/modules</code>에 있는 리포지토리를 가리키는 gitlink를 workdir에 포함해야 하는지 여부</p>
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
    <p class="param-description">리포지토리</p>
  </li>
</ul>