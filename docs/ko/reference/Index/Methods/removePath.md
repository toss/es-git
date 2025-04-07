# removePath

디스크상의 파일에 해당하는 인덱스 항목을 제거해요.

만약 이 파일이 현재 병합 충돌의 결과라면, 더 이상 충돌 상태로 표시되지 않아요. 충돌에 관한 데이터는 "resolve undo" (REUC) 섹션으로 이동돼요.

## 시그니처

```ts
class Index {
  removePath(path: string, options?: IndexRemoveOptions | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">파일 경로는 리포지토리의 작업 폴더에 상대적이어야 하며 읽을 수 있어야 해요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | IndexRemoveOptions</span>
    <br>
    <p class="param-description">인덱스 항목을 제거할 때 사용할 옵션이에요.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">stage</span><span class="param-type">IndexStage</span>
        <br>
        <p class="param-description">- <code>Any</code> : 모든 인덱스 스테이지와 일치.<br>- <code>Normal</code> : 인덱스에 정상적으로 스테이징된 파일.<br>- <code>Ancestor</code> : 충돌 시 조상 측 파일.<br>- <code>Ours</code> : 충돌 시 &quot;우리&quot; 측 파일.<br>- <code>Theirs</code> : 충돌 시 &quot;상대&quot; 측 파일.</p>
      </li>
    </ul>
  </li>
</ul>
