# addToIndex

현재 서브모듈 HEAD 커밋을 상위 프로젝트의 인덱스에 추가해요.

## 시그니처

```ts
class Submodule {
  addToIndex(writeIndex?: boolean | null | undefined): void;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">writeIndex</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">true이면 인덱스 파일이 즉시 기록되는 것. 그렇지 않으면 나중에 <code>Index</code>에서 <code>write()</code>를 명시적으로 호출해야 하는 것</p>
  </li>
</ul>