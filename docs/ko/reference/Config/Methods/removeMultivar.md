# removeMultivar

가장 높은 수준(보통 로컬)의 설정 파일에서 multivar 설정 변수를 제거해요.

## 시그니처

```ts
class Config {
  removeMultivar(name: string, regexp: string): void;
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
    <span class="param-name">regexp</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">값에 대해 대소문자를 구분하여 적용되는 정규 표현식.</p>
  </li>
</ul>