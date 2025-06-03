# getAnnotatedCommitFromFetchHead

`FETCH_HEAD`에서 Annotated Commit을 생성해요.

## 시그니처

```ts
class Repository {
  getAnnotatedCommitFromFetchHead(
    branchName: string,
    remoteUrl: string,
    id: string,
  ): AnnotatedCommit;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">branchName</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">원격 브랜치의 이름</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">remoteUrl</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">원격의 URL</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">원격 브랜치의 커밋 개체 ID</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description"><code>FETCH_HEAD</code>에서 생성된 Annotated Commit</p>
  </li>
</ul>