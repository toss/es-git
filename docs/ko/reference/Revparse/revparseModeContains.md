# revparseModeContains

주어진 `source` 값이 특정 `target` 플래그를 포함하는지 확인해요.

## 시그니처

```ts
function revparseModeContains(source: number, target: number): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">source</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">검사할 원본 플래그 값이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description"><code>source</code> 값이 포함해야 할 대상 플래그 값이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description"><code>source</code> 값이 <code>target</code> 플래그를 포함하면 <code>true</code>를 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository, RevparseMode } from 'es-git';

const repo = await openRepository('.');
const spec = repo.revparse('main..other');

console.assert(spec.mode, RevparseMode.Range);
```