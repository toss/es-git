# remove

reflog에서 항목을 제거해요.

## 시그니처

```ts
class Reflog {
  remove(i: number, rewritePreviousEntry?: boolean): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">i</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">제거할 항목의 인덱스</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">rewritePreviousEntry</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">이전 항목을 다시 쓸지 여부예요. 기본값은 <code>false</code>예요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">인덱스가 유효하지 않거나 제거에 실패하면 오류를 던져요.</p>
  </li>
</ul>