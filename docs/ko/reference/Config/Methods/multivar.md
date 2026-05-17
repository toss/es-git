# multivar

multivar의 값을 반복해요.

## 시그니처

```ts
class Config {
  multivar(name: string, regexp?: string): ConfigEntries;
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
    <span class="param-name">regexp</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description"><code>regexp</code>가 제공되면, 반복자는 패턴과 일치하는 값만 반복해요. 정규 표현식은 변수 이름의 정규화된 형식에 대해 대소문자를 구분하여 적용돼요: 섹션과 변수 부분은 소문자로 변환돼요. 하위 섹션은 변경되지 않아요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ConfigEntries</span>
    <br>
    <p class="param-description">구성의 <code>ConfigEntry</code> 값에 대한 반복자.</p>
  </li>
</ul>