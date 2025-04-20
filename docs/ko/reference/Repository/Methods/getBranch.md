# getBranch

리포지토리에서 주어진 이름으로 브랜치를 조회해요.

## 시그니처

```ts
class Repository {
  getBranch(name: string, branchType: BranchType): Branch;
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
    <p class="param-description">- <code>Local</code> : 원격에 없는 로컬 브랜치예요.<br>- <code>Remote</code> : 원격의 브랜치예요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">찾은 브랜치예요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">브랜치가 존재하지 않으면 오류를 발생해요.</p>
  </li>
</ul>