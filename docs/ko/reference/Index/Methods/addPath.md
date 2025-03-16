# addPath

디스크상의 파일로부터 인덱스 항목을 추가 또는 업데이트해요.

이 메소드는 gitignore 규칙을 무시하고 파일을 인덱스에 추가하도록 강제해요.

현재 이 파일이 병합 충돌의 결과라면, 더 이상 충돌 상태로 표시되지 않고, 충돌에 관한 데이터는 "resolve undo" (REUC) 영역으로 이동돼요.

## 시그니처

```ts
class Index {
  addPath(path: string): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">파일 경로는 리포지토리의 작업 폴더에 상대적이어야 하며 읽을 수 있어야 해요.</p>
  </li>
</ul>

### 에러

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description"><code>bare</code> 인덱스 인스턴스에서는 이 메소드가 실패할 거예요.</p>
  </li>
</ul>