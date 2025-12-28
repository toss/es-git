# applyToTree

제공된 트리에 Diff를 적용하고, 결과 Index를 반환해요.

## 시그니처

```ts
class Repository {
  applyToTree(
    tree: Tree,
    diff: Diff,
    options?: ApplyOptions | null | undefined
  ): Index;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">diff를 적용할 트리</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">적용할 diff</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | ApplyOptions</span>
    <br>
    <p class="param-description">apply에 대한 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">check</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">실제로 변경하지 않고, 패치가 적용되는지만 테스트해요.</p>
      </li>
    </ul>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">적용 결과의 postimage</p>
  </li>
</ul>