# setI32

가장 높은 수준(보통 로컬)의 구성 파일에서 정수 구성 변수의 값을 설정해요.

## 시그니처

```ts
class Config {
  setI32(name: string, value: number): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">구성 항목의 이름.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">구성 항목의 값.</p>
  </li>
</ul>