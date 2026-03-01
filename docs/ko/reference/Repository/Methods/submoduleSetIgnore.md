# submoduleSetIgnore

구성에서 서브모듈에 대한 무시 규칙을 설정해요.

이 작업은 현재 로드된 어떤 인스턴스에도 영향을 주지 않아요.

## 시그니처

```ts
class Repository {
  submoduleSetIgnore(name: string, ignore: SubmoduleIgnore): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">서브모듈의 이름</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ignore</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">무시 규칙에 대한 새 값</p>
    <p class="param-description">서브모듈 무시 값<br><br>이 값들은 <code>submodule.$name.ignore</code> 구성 값에 대한 설정을 나타내요.<br>이 구성 값은 서브모듈 상태를 가져올 때 작업 디렉터리를 얼마나 깊게 살펴볼지 알려줘요.</p>
  </li>
</ul>