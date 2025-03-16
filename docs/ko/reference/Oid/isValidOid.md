# isValidOid

주어진 문자열이 유효한 OID(Object ID)인지 확인해요.

## 시그니처

```ts
function isValidOid(value: string): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">유효한 OID인지 확인할 문자열이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">
      문자열이 비어 있거나, 40자리 16진수보다 길거나, 16진수 이외의 문자가 포함되어 있으면 <code>false</code>를 반환해요.
    </p>
  </li>
</ul>