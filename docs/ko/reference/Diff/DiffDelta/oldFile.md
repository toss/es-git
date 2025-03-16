# oldFile

Diff에서 "from" 쪽을 나타내는 파일을 반환해요.

어느 쪽을 의미하는지는 diff를 생성하는 데 사용된 함수에 따라 달라지며, 해당 함수 자체의 문서에서 설명돼 있어요.

## 시그니처

```ts
class DiffDelta {
  oldFile(): DiffFile;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DiffFile</span>
    <br>
    <p class="param-description">Diff에서 &quot;from&quot; 쪽을 나타내는 파일을 반환해요.</p>
  </li>
</ul>