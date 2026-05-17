# parseConfigBool

문자열을 bool로 파싱해요.

## 시그니처

```ts
function parseConfigBool(value: string): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">입력 값.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">&quot;true&quot;, &quot;yes&quot;, &quot;on&quot;, 1, 또는 0이 아닌 숫자를 <code>true</code>로 해석해요.<br>&quot;false&quot;, &quot;no&quot;, &quot;off&quot;, 0, 또는 빈 문자열을 <code>false</code>로 해석해요.</p>
  </li>
</ul>