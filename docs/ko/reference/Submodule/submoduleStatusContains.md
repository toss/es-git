# submoduleStatusContains

서브모듈 상태에 지정된 값이 포함되어 있는지 확인해요.

## 시그니처

```ts
function submoduleStatusContains(source: number, target: number): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">source</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">소스 상태</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">대상 상태</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">source 상태에 target 상태가 포함되어 있으면 <code>true</code>를 반환</p>
  </li>
</ul>