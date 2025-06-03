# setHeadDetachedFromAnnotated

리포지토리 HEAD가 커밋을 직접 가리키도록 만들어요.

제공된 commitish를 리포지토리에서 찾을 수 없으면 HEAD는 변경되지 않고 오류가 반환돼요.
제공된 commitish를 커밋으로 벗길 수 없으면 HEAD는 변경되지 않고 오류가 반환돼요.
그렇지 않으면 HEAD는 결국 분리되어 벗겨진 커밋을 직접 가리키게 돼요.

## 시그니처

```ts
class Repository {
  setHeadDetachedFromAnnotated(commitish: AnnotatedCommit): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitish</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">HEAD가 가리켜야 하는 주석이 달린 커밋</p>
  </li>
</ul>