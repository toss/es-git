# open

하위 모듈의 리포지토리를 열어요.

이 기능은 하위 모듈이 작업 디렉터리에 체크아웃되어 있는 경우에만 동작해요.

## 시그니처

```ts
class Submodule {
  open(signal?: AbortSignal | null | undefined): Promise<Repository>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">작업을 취소하기 위한 선택적 AbortSignal</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">리포지토리</p>
  </li>
</ul>