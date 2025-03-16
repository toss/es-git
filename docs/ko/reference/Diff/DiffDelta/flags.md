# flags

## 시그니처

```ts
class DiffDelta {
  flags(): number;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">Delta에 설정된 플래그들을 반환해요.</p>
  </li>
</ul>

## 예제

```ts
import { DiffDelta, DiffFlags, diffFlagsContains } from 'es-git';

const delta: DiffDelta;
console.assert(diffFlagsContains(delta.flags(), DiffFlags.Binary | DiffFlags.ValidId));
```
