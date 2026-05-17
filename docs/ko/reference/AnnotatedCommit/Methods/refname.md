# refname

주어진 Annotated Commit이 참조하는 refname을 가져와요.

## 시그니처

```ts
class AnnotatedCommit {
  refname(): string | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">이 Annotated Commit이 참조하는 refname. 참조로부터 생성된 경우, 반환값은 <code>null</code> 이에요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">refname이 유효한 utf-8이 아닌 경우 오류를 던져요.</p>
  </li>
</ul>