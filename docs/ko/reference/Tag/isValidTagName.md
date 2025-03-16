# isValidTagName

태그 이름이 유효한지 확인해요. 즉, 태그 이름이 `refs/tags/`로 시작했을 때 올바른 참조 이름인지, 그리고 추가적인 태그 이름 제한 사항(예: `-`로 시작 불가)을 충족하는지 판단해요.

## 시그니처

```ts
function isValidTagName(tagName: string): boolean;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tagName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">유효성을 검사할 태그 이름이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">태그 이름이 유효하면 <code>true</code>를 반환해요.</p>
  </li>
</ul>
