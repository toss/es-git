# walk

트리(tree)와 해당 서브 트리(subtree)의 각 항목을 전위(pre-order) 또는 후위(post-order) 순서로 순회해요.

## 시그니처

```ts
class Tree {
  walk(mode: TreeWalkMode, callback: (entry: TreeEntry) => number): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">mode</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">TreeWalkMode</span>
    <br>
    <p class="param-description">트리를 순회할 때 전위(pre-order) 순회와 후위(post-order) 순회 중 어떤 순서를 사용할지를 결정해요.</p>
    <p class="param-description">
      - <code>PreOrder</code>: 전위(pre-order) 방식으로 트리를 순회해요.<br>
      - <code>PostOrder</code>: 후위(post-order) 방식으로 트리를 순회해요.
    </p>
  </li>

  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">(entry: TreeEntry) =&gt; number</span>
    <br>
    <p class="param-description">순회 중 각 항목을 방문할 때마다 실행되는 콜백 함수예요. <code>libgit2</code>는 이 콜백에서 돌려주는 값을 보고 순회를 계속 진행할지, 생략할지 또는 중단할지를 결정해요.</p>
    <p class="param-description">
      반환 값의 의미는 다음과 같아요:<br>
      - <code>0</code>: 방문 성공, 순회를 계속 진행<br>
      - <code>1</code>: 현재 항목을 건너뛰고 계속 진행<br>
      - <code>-1</code>: 순회를 즉시 중단
    </p>
  </li>
</ul>
