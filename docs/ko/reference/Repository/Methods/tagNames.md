# tagNames

리포지토리에 있는 모든 태그 목록을 가져와요.

## 시그니처

```ts
class Repository {
  tagNames(pattern?: string | null | undefined): string[];
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">pattern</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">선택적으로 <code>fnmatch</code> 패턴을 지정할 수 있어요. 지정하면 해당 패턴과 일치하는 태그만 반환돼요.</p>
  </li>
</ul>
