# findBranch

리포지토리에서 주어진 이름으로 브랜치를 찾아요.

## 시그니처

```ts
class Repository {
  findBranch(name: string, branchType: BranchType): Branch | null;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">찾을 브랜치 이름이에요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">branchType</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">BranchType</span>
    <br>
    <p class="param-description">조회할 브랜치 유형을 설정해요.</p>
    <p class="param-description">- <code>Local</code> : 리모트에 없는 로컬 브랜치에요.<br>- <code>Remote</code> : 리모트를 위한 브랜치에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Branch</span>
    <br>
    <p class="param-description">찾은 브랜치예요.</p>
  </li>
</ul>