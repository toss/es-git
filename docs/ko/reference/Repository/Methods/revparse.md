# revparse

주어진 리비전 표현식을 해석해요.

결과로 변환된 리비전(revspec) 정보를 반환해요.

## 시그니처

```ts
class Repository {
  revparse(spec: string): Revspec;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">spec</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">해석할 리비전 문자열이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Revspec</span>
    <br>
    <p class="param-description">해석된 리비전 정보를 포함하는 객체예요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">from</span><span class="param-type">string</span>
        <br>
        <p class="param-description">이 리비전 스펙에서 시작 지점을 나타내요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">리비전 스펙의 해석 모드를 나타내요.</p>
      </li>
      <li class="param-li">
        <span class="param-name">to</span><span class="param-type">string</span>
        <br>
        <p class="param-description">이 리비전 스펙에서 끝 지점을 나타내요.</p>
      </li>
    </ul>
  </li>
</ul>