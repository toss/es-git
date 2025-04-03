# asCommit

이 개체를 커밋으로 간주하려고 시도해요.

## 시그니처

```ts
class GitObject {
  asCommit(): Commit | null;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Commit</span>
    <br>
    <p class="param-description">만약 해당 개체가 실제 커밋이 아니라면 <code>null</code>을 반환해요.</p>
  </li>
</ul>