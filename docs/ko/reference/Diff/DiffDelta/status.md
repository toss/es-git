# status

이 항목의 상태를 반환해요.

## 시그니처

```ts
class DiffDelta {
  status(): DeltaType;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DeltaType</span>
    <br>
    <p class="param-description">엔트리의 상태를 반환해요.</p>
    <p class="param-description">- <code>Unmodified</code> : 변화 없음.<br>- <code>Added</code> : 새로 추가됨.<br>- <code>Deleted</code> : 제거됨.<br>- <code>Modified</code> : 변경됨.<br>- <code>Renamed</code> : 이름이 변경됨.<br>- <code>Copied</code> : 이전 엔트리로부터 복사됨.<br>- <code>Ignored</code> : 작업 디렉터리에서 무시됨.<br>- <code>Untracked</code> : 작업 디렉터리에서 추적되지 않음.<br>- <code>Typechange</code> : 타입 변경됨.<br>- <code>Unreadable</code> : 읽을 수 없음.<br>- <code>Conflicted</code> : Index에서 컨플릭 상태</p>
  </li>
</ul>