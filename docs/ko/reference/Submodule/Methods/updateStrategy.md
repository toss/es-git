# updateStrategy

서브모듈에 사용될 업데이트 규칙을 가져와요.

## 시그니처

```ts
class Submodule {
  updateStrategy(): SubmoduleUpdate;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">SubmoduleUpdate</span>
    <br>
    <p class="param-description">서브모듈에 사용될 업데이트 규칙</p>
    <p class="param-description">서브모듈 업데이트 값<br><br>이 값들은 <code>submodule.$name.update</code> 구성 값에 대한 설정을 나타내며, 이 구성 값은 이 서브모듈에 대해 <code>git submodule update</code>를 어떻게 처리할지 말해요. 값은 보통 &quot;.gitmodules&quot; 파일에 설정되고, 서브모듈이 초기화될 때 &quot;.git/config&quot;로 복사돼요.</p>
  </li>
</ul>