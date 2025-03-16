# name

트리 항목(tree entry)의 파일 이름을 반환해요.

## 시그니처

```ts
class TreeEntry {
  name(): string;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">트리 항목의 파일 이름이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">파일 이름이 올바른 UTF-8 형식이 아닐 때 오류를 발생시켜요.</p>
  </li>
</ul>
