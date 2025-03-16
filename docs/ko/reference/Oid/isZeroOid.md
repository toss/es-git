# isZeroOid

주어진 OID(Object ID)가 모두 0으로 이루어져 있는지 확인해요.

## 시그니처

```ts
function isZeroOid(value: string): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">0으로만 구성된 OID인지 확인할 문자열이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">
      문자열이 0으로만 이루어져 있으면 <code>true</code>를 반환해요.
    </p>
  </li>
</ul>

## 예제

```ts
import { zeroOid, isZeroOid } from 'es-git';

console.assert(isZeroOid(zeroOid());
```