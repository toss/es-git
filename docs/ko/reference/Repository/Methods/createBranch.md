# createBranch

특정 커밋을 가리키는 새로운 브랜치를 생성해요.

## 시그니처

```ts
class Repository {
  createBranch(
    branchName: string,
    target: Commit,
    options?: CreateBranchOptions | null | undefined,
  ): Branch;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">branchName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">새로운 브랜치의 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">이 브랜치가 가리킬 대상 커밋이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateBranchOptions</span>
    <br>
    <p class="param-description">브랜치를 생성할 때의 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description"><code>force</code>가 true면, 주어진 이름으로 이미 참조가 존재할 경우 그것이 대체돼요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">새로 생성된 브랜치예요.</p>
  </li>
</ul>