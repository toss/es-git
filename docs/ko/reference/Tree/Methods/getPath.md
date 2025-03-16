# getPath

상대 경로를 사용하여 트리 또는 하위 트리에 포함된 트리 항목을 가져와요.

## 시그니처

```ts
class Tree {
  getPath(path: string): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">트리 항목의 상대 경로예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">상대 경로와 일치하는 트리 항목이에요. 일치하는 항목이 없으면 <code>null</code>이 반환돼요.</p>
  </li>
</ul>
