# rename

기존 로컬 브랜치의 이름을 변경해요.

## 시그니처

```ts
class Branch {
  rename(newBranchName: string, options?: BranchRenameOptions | null | undefined): Branch;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newBranchName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">변경할 브랜치 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | BranchRenameOptions</span>
    <br>
    <p class="param-description">이름 변경 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>force</code> 옵션이 활성화되지 않았고, 주어진 이름으로 브랜치가 이미 존재하면 이름 변경이 실패해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">이름이 변경된 브랜치에요.</p>
  </li>
</ul>