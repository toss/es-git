# hashObjectOid

주어진 데이터를 지정된 Git 개체 타입으로 해시하고, 그 결과에 해당하는 OID(Object ID)를 반환해요.  
이 함수는 해당 개체를 어떤 개체 데이터베이스나 저장소에 저장하지 않아요.

## 시그니처

```ts
function hashObjectOid(objType: ObjectType, bytes: Buffer): string;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">objType</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">ObjectType</span>
    <br>
    <p class="param-description">Git 개체 타입이에요.</p>
    <p class="param-description">
      - <code>Any</code> : 모든 종류의 Git 개체<br>
      - <code>Commit</code> : Git 커밋에 해당하는 개체<br>
      - <code>Tree</code> : Git 트리에 해당하는 개체<br>
      - <code>Blob</code> : Git 블롭(blob)에 해당하는 개체<br>
      - <code>Tag</code> : Git 태그에 해당하는 개체
    </p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">bytes</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
    <br>
    <p class="param-description">해시할 데이터예요.</p>
  </li>
</ul>


### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">해시된 문자열을 반환해요.</p>
  </li>
</ul>