# deleteTag

기존 태그 레퍼런스를 삭제해요.

## 시그니처

```ts
class Repository {
  deleteTag(name: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">
      삭제할 태그의 이름이에요.  
      태그 이름의 유효성 검사는 <code>isValidTagName</code>을 참고하세요.
    </p>
  </li>
</ul>
