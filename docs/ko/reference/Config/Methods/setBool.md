# setBool

가장 높은 수준(보통 로컬)의 설정 파일에서 boolean 설정 변수의 값을 설정해요.

## 시그니처

```ts
class Config {
  setBool(name: string, value: boolean): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">설정 항목의 이름.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
    <br>
    <p class="param-description">설정 항목의 값.</p>
  </li>
</ul>