# apply

주어진 리포지토리에 Diff를 적용해서 작업 디렉터리, 인덱스 또는 둘 다에 직접 변경 사항을 반영해요.

## 시그니처

```ts
class Repository {
  apply(diff: Diff, location: ApplyLocation, options?: ApplyOptions | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">적용할 diff</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">location</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">ApplyLocation</span>
    <br>
    <p class="param-description">적용할 위치</p>
    <p class="param-description">git_apply의 가능한 적용 위치<br>&lt;https://libgit2.org/libgit2/#HEAD/type/git_apply_options&gt; 참고</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | ApplyOptions</span>
    <br>
    <p class="param-description">apply에 대한 옵션</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">check</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">실제로 변경 사항을 만들지 않고 패치가 적용되는지만 테스트해요.</p>
      </li>
    </ul>
  </li>
</ul>