# peel

특정 타입의 개체가 나올 때까지 재귀적으로 개체를 풀어요.

## 시그니처

```ts
class GitObject {
  peel(objType: ObjectType): GitObject;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">objType</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">ObjectType</span>
    <br>
    <p class="param-description">만약 대상 타입으로 <code>Any</code>를 전달하면, 타입이 바뀔 때까지 (예: 태그인 경우 참조된 개체가 더 이상 태그가 아닐 때까지) 개체를 풀어요.</p>
    <p class="param-description">
  - <code>Any</code> : 모든 종류의 Git 개체<br>
  - <code>Commit</code> : Git 커밋에 해당하는 개체<br>
  - <code>Tree</code> : Git 트리에 해당하는 개체<br>
  - <code>Blob</code> : Git 블롭(blob)에 해당하는 개체<br>
  - <code>Tag</code> : Git 태그에 해당하는 개체
</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">재귀적으로 펼쳐진 Git 개체예요.</p>
  </li>
</ul>