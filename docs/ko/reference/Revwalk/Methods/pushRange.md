# pushRange

주어진 커밋 범위의 시작점을 숨기고 끝점을 탐색 시작점으로 추가해요.

## 시그니처

```ts
class Revwalk {
  pushRange(range: string): this;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">range</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description"><code>&lt;commit&gt;..&lt;commit&gt;</code> 형식의 커밋 범위예요.  
      각 <code>&lt;commit&gt;</code>은 <code>revparseSingle</code>에서 사용할 수 있는 형식이어야 해요.  
      왼쪽 커밋은 숨겨지고, 오른쪽 커밋이 Revwalk의 탐색 시작점으로 추가돼요.
</p>
  </li>
</ul>