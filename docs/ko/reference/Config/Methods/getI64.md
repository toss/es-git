# getI64

정수 구성 변수의 값을 가져와요.

모든 구성 파일은 정의된 수준의 순서대로 조회돼요. 높은 수준일수록 높은 우선순위를 가져요. 변수의 첫 번째 항목이 여기에 반환돼요.

## 시그니처

```ts
class Config {
  getI64(name: string): number;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">구성 항목의 이름.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">정수 구성 변수의 값.</p>
  </li>
</ul>