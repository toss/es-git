# getAnnotatedCommit

주어진 커밋으로부터 Annotated Commit을 생성해요.

## 시그니처

```ts
class Repository {
  getAnnotatedCommit(commit: Commit): AnnotatedCommit;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commit</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">Annotated Commit을 생성할 커밋</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">커밋으로부터 생성된 Annotated Commit</p>
  </li>
</ul>