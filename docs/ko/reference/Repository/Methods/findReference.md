# findReference

리포지토리에서 특정 레퍼런스를 조회해요.

## 시그니처

```ts
class Repository {
  findReference(name: string): Reference | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">조회할 레퍼런스의 이름이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Reference</span>
    <br>
    <p class="param-description">주어진 이름에 해당하는 레퍼런스를 반환해요. 레퍼런스가 존재하지 않으면 <code>null</code>을 반환해요.</p>
  </li>
</ul>

## 예제

리포지토리에서 `HEAD` 레퍼런스를 가져와요.

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const reference = repo.findReference('HEAD');
```