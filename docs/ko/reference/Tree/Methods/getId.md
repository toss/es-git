# getId

SHA 값을 이용해 트리 항목을 찾아와요.

## 시그니처

```ts
class Tree {
  getId(id: string): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">트리 항목의 SHA 값이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">지정된 SHA(ID)를 갖는 트리 항목이에요. 해당 ID를 가진 항목이 없다면 <code>null</code>이 반환돼요.</p>
  </li>
</ul>
