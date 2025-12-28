# submoduleSetUpdate

구성에서 서브모듈의 업데이트 규칙을 설정해요

이 설정은 기존 인스턴스에는 영향을 주지 않아요.

## 시그니처

```ts
class Repository {
  submoduleSetUpdate(name: string, update: SubmoduleUpdate): void;
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
    <span class="param-name">update</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleUpdate</span>
    <br>
    <p class="param-description">사용할 새 값</p>
    <p class="param-description">서브모듈 업데이트 값<br><br>이 값들은 이 서브모듈에 대해 <code>git submodule update</code>를 어떻게 처리할지 말하는 <code>submodule.$name.update</code> 구성 값의 설정을 나타내는 값<br>이 값은 보통 &quot;.gitmodules&quot; 파일에 설정되고, 서브모듈이 초기화될 때 &quot;.git/config&quot;로 복사되는 값</p>
  </li>
</ul>