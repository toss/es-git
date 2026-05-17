# prune

워크트리를 정리해요.

## 시그니처

```ts
class Worktree {
  prune(options?: WorktreePruneOptions | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">worktreePruneOptions</span><span class="param-type">WorktreePruneOptions | null</span>
    <br>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">locked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">잠긴 워크트리를 정리할지 여부를 제어해요. 기본값은 <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">valid</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">유효한 (파일 시스템에 여전히 존재하는) 워크트리를 정리할지 여부를 제어해요. 기본값은 <code>false</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">workingTree</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">파일 시스템의 실제 작업 트리를 재귀적으로 제거할지 여부를 제어해요. 기본값은 <code>false</code>.</p>
      </li>
    </ul>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">정리에 실패하면 오류를 발생시켜요.</p>
  </li>
</ul>