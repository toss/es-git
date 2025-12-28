# reflogRename

reflog의 이름을 변경해요.

## 시그니처

```ts
class Repository {
  reflogRename(oldName: string, newName: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oldName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">참조의 이전 이름</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">newName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">참조의 새 이름</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">이름 변경에 실패하면 오류를 던져요.</p>
  </li>
</ul>