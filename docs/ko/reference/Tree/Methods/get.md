# get

트리에서 특정 위치의 트리 항목을 찾아와요.

## 시그니처

```ts
class Tree {
  get(index: number): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">가져오려는 트리 항목의 위치(인덱스)입니다.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">해당 위치에 존재하는 트리 항목이에요. 해당 위치에 항목이 없으면 <code>null</code>이 반환돼요.</p>
  </li>
</ul>
