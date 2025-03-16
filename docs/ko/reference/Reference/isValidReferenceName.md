# isValidReferenceName

레퍼런스 이름이 올바른 형식인지 확인해요.

검사는 `normalizeReferenceName` 함수에서 `ReferenceFormat.AllowOnelevel` 옵션을 사용한 것과 동일하게 수행돼요.  
다만, 이 함수는 레퍼런스 이름을 정규화(normalization)하지 않아요.

## 시그니처

```ts
function isValidReferenceName(refname: string): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">유효한 레퍼런스 이름인지 확인할 문자열이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">
      레퍼런스 이름이 올바르면 <code>true</code>를 반환해요.
    </p>
  </li>
</ul>

## 예제

```ts
import { isValidReferenceName } from 'es-git';

console.assert(isValidReferenceName("HEAD"));
console.assert(isValidReferenceName("refs/heads/main"));

// But:
console.assert(!isValidReferenceName("main"));
console.assert(!isValidReferenceName("refs/heads/*"));
console.assert(!isValidReferenceName("foo//bar"));
```