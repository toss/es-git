# ignoreRule

서브모듈에 사용될 ignore 규칙을 가져와요.

## 시그니처

```ts
class Submodule {
  ignoreRule(): SubmoduleIgnore;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">서브모듈에 사용될 ignore 규칙</p>
    <p class="param-description">서브모듈 ignore 값<br><br>이 값들은 <code>submodule.$name.ignore</code> 설정 값에 대한 설정을 나타내며, 서브모듈 상태를 가져올 때 작업 디렉터리를 얼마나 깊게 살펴볼지를 말해요.</p>
  </li>
</ul>