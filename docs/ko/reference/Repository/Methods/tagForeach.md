# tagForeach

리포지토리에 있는 모든 태그를 순회하며 `callback`을 실행해요.  
콜백 함수는 태그 ID(OID)와 태그 이름을 인자로 받아요.

## 시그니처

```ts
class Repository {
  tagForeach(callback: (oid: string, name: string) => boolean): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">(oid: string, name: string) =&gt; boolean</span>
    <br>
    <p class="param-description">각 태그에 대해 호출되는 콜백 함수예요. 순회를 중단하려면 콜백에서 <code>false</code>를 반환하면 돼요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const tags = [];
repo.tagForeach((sha, name) => {
  tags.push([name, sha]);
  return true;
});

console.log(tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
//  ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2']]
```