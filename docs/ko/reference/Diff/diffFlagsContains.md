# diffFlagsContains

diff 플래그가 특정 플래그를 포함하는지 확인해요.

## 시그니처

```ts
function diffFlagsContains(source: number, target: number): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">source</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">소스 플래그.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">대상 플래그.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">소스 플래그가 대상 플래그를 포함한다면 <code>true</code>를 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { DiffDelta, DiffFlags, diffFlagsContains } from 'es-git';

const delta: DiffDelta;
console.assert(diffFlagsContains(delta.flags(), DiffFlags.Binary | DiffFlags.ValidId));
```