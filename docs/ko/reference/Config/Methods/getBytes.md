# getBytes

문자열 config 변수의 값을 바이트 슬라이스로 가져와요.

## 시그니처

```ts
class Config {
  getBytes(name: string): Uint8Array;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">config 항목의 이름.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Uint8Array</span>
    <br>
    <p class="param-description">바이트 슬라이스로 표현된 문자열 config 변수의 값.</p>
  </li>
</ul>