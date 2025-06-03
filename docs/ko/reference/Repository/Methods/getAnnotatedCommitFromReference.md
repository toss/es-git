# getAnnotatedCommitFromReference

주어진 참조에서 주석이 달린 커밋을 생성해요.

## 시그니처

```ts
class Repository {
  getAnnotatedCommitFromReference(reference: Reference): AnnotatedCommit;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reference</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">GitReference</span>
    <br>
    <p class="param-description">주석이 달린 커밋을 생성할 참조</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">참조에서 생성된 주석이 달린 커밋</p>
  </li>
</ul>