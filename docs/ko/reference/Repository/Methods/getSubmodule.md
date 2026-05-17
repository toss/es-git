# getSubmodule

이름 또는 경로로 서브모듈 정보를 조회해요.

서브모듈 이름 또는 경로(보통 동일함)가 주어지면,
서브모듈을 설명하는 구조체를 반환해요.

## 시그니처

```ts
class Repository {
  getSubmodule(name: string): Submodule;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서브모듈의 이름 또는 경로; 후행 슬래시 허용.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Submodule</span>
    <br>
    <p class="param-description">서브모듈.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">서브모듈을 찾을 수 없는 경우.</p>
  </li>
</ul>