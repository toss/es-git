# normalizeReferenceName

레퍼런스 이름을 정규화하고 유효성을 검사해요.

이 함수는 레퍼런스 이름에서 인접한 슬래시(`/`)가 여러 개 연속되면 하나의 슬래시로 변환해요.  
또한, 다음 규칙에 따라 이름의 유효성을 검사해요.

1. `ReferenceFormat.AllowOnelevel`이 주어진 경우
   - 이름에는 대문자와 밑줄(`_`)만 포함될 수 있어요.
   - 반드시 문자로 시작하고 문자로 끝나야 해요. (예: `"HEAD"`, `"ORIG_HEAD"`)

2. `ReferenceFormat.RefspecShorthand`이 `ReferenceFormat.AllowOnelevel`과 함께 사용된 경우
   - 단일 단어로 된 브랜치 이름(예: `"main"`)도 허용돼요.
   - 단, `refs/` 접두사가 없고, `/`가 포함되지 않아야 해요.

3. `ReferenceFormat.RefspecPattern`이 주어진 경우
   - 전체 경로 요소를 대신하는 `*`가 한 번 포함될 수 있어요.
   - 예: `"foo/*/bar"`, `"foo/bar*"`

4. `"refs/"`로 시작하는 이름은 대부분 허용돼요.
   - 다만, `~`, `^`, `:`, `\`, `?`, `[`, `*`와 같은 문자와 `".."`, `"@{"`와 같은 문자열은 사용할 수 없어요.
   - 이러한 문자는 `revparse`(Git의 참조 파싱)에서 특별한 의미를 가지기 때문이에요.

## 시그니처

```ts
function normalizeReferenceName(refname: string, format?: number): string | null;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">정규화할 레퍼런스 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">format</span><span class="param-type">null | number</span>
    <br>
    <p class="param-description">정규화에 사용할 레퍼런스 포맷 플래그예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">
      레퍼런스가 유효하면 정규화된 형태로 반환해요.  
      유효하지 않으면 <code>null</code>을 반환해요.
    </p>
  </li>
</ul>

## 예제

```ts
import { normalizeReferenceName, ReferenceFormat } from 'es-git';

console.assert(
  normalizeReferenceName('foo//bar'),
  'foo/bar'
);
console.assert(
  normalizeReferenceName(
    'HEAD',
    ReferenceFormat.AllowOnelevel
  ),
  'HEAD'
);
console.assert(
  normalizeReferenceName(
    'refs/heads/*',
    ReferenceFormat.RefspecPattern
  ),
  'refs/heads/*'
);
console.assert(
  normalizeReferenceName(
    'main',
    ReferenceFormat.AllowOnelevel | ReferenceFormat.RefspecShorthand
  ),
  'main'
);
```