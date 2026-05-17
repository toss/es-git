# parseConfigI64

문자열을 i64로 파싱해요; k, M, G와 같은 접미사를 처리하고 1024의 적절한 거듭제곱을 곱해요.

## 시그니처

```ts
function parseConfigI64(value: string): number;
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
    <span class="param-type">number</span>
    <br>
    <p class="param-description">파싱된 i64 값.</p>
  </li>
</ul>