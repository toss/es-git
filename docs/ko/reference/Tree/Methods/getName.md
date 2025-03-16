# getName

파일 이름을 이용해 트리 항목을 찾아와요.

## 시그니처

```ts
class Tree {
  getName(filename: string): TreeEntry | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">filename</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">찾아올 트리 항목의 파일 이름이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">주어진 파일 이름에 해당하는 트리 항목이에요. 일치하는 항목이 없으면 <code>null</code>이 반환돼요.</p>
  </li>
</ul>
