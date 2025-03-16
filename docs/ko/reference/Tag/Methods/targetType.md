# targetType

태그가 가리키는 Git 개체의 타입을 가져와요.

## 시그니처

```ts
class Tag {
  targetType(): ObjectType | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ObjectType</span>
    <br>
    <p class="param-description">태그가 가리키는 Git 개체의 타입이에요.</p>
    <p class="param-description">- <code>Any</code> : 모든 종류의 Git 개체<br>- <code>Commit</code> : Git 커밋에 해당하는 개체<br>- <code>Tree</code> : Git 트리에 해당하는 개체<br>- <code>Blob</code> : Git 블롭에 해당하는 개체<br>- <code>Tag</code> : Git 태그에 해당하는 개체</p>
  </li>
</ul>