# getI32

정수 설정 변수의 값을 가져와요.

모든 설정 파일은 정의된 수준의 순서대로 조회돼요. 높은 수준은 높은 우선순위를 의미해요. 변수의 첫 번째 항목이 여기에 반환돼요.

## 시그니처

```ts
class Config {
  getI32(name: string): number;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">설정 항목의 이름.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">정수 설정 변수의 값.</p>
  </li>
</ul>