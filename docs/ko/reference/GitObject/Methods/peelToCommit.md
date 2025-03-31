# peelToCommit

커밋이 나올 때까지 재귀적으로 개체를 풀어요.

## 시그니처

```ts
class GitObject {
  peelToCommit(): Commit;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Commit</span>
    <br>
    <p class="param-description">커밋을 반환해요.</p>
  </li>
</ul>