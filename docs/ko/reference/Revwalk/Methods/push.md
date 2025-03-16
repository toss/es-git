# push

탐색을 시작할 커밋을 추가해요.

주어진 OID는 Revwalk가 순회할 리포지토리에 속한 커밋이어야 해요. 이 커밋은 탐색의 시작점으로 사용돼요.  

Revwalk를 시작하려면 최소 하나 이상의 커밋을 `push`해야 해요.

## 시그니처

```ts
class Revwalk {
  push(oid: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Revwalk에서 탐색할 리포지토리에 속한 커밋의 OID예요.</p>
  </li>
</ul>