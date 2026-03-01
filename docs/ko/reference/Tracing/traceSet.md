# traceSet

libgit2에서 추적 메시지를 생성할 때 호출되는 전역 구독자를 설정해요.

## 시그니처

```ts
function traceSet(
  level: TraceLevel,
  callback: (level: TraceLevel, message: string) => void,
): void;
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">level</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">TraceLevel</span>
    <br>
    <p class="param-description">추적을 설정할 레벨</p>
    <p class="param-description">사용 가능한 추적 레벨. 추적을 특정 레벨로 설정하면 호출자는 해당 레벨과 그보다 낮은 모든 레벨에서의 추적을 제공받아요.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">필수</span>&nbsp;·&nbsp;<span class="param-type">(level: TraceLevel, message: string) =&gt; void</span>
    <br>
    <p class="param-description">추적 데이터와 함께 호출할 콜백</p>
  </li>
</ul>