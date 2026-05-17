# entries

모든 config 변수를 반복해요.

## 시그니처

```ts
class Config {
  entries(glob?: string): ConfigEntries;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">glob</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description"><code>glob</code>이 제공되면, 이터레이터는 이름이 패턴과 일치하는 변수만 반복해요. 정규 표현식은 변수 이름의 정규화된 형태에 대해 대소문자를 구분하여 적용돼요: 섹션과 변수 부분은 소문자로 변환돼요. 하위 섹션은 변경되지 않아요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ConfigEntries</span>
    <br>
    <p class="param-description">config의 <code>ConfigEntry</code> 값에 대한 이터레이터.</p>
  </li>
</ul>

## 예제

```ts
import { openDefaultConfig } from 'es-git';

const config = openDefaultConfig();
for (const entry of config.entries()) {
  console.log(entry.name, entry.value);
}
```