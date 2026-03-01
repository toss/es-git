# reload

config, index, `HEAD`에서 서브모듈 정보를 다시 읽어요.

이 서브모듈의 캐시된 서브모듈 정보가 변경되었을 가능성이 있다고 생각된다면, 이를 다시 읽기 위해 호출하세요.

## 시그니처

```ts
class Submodule {
  reload(
    force?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">force</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">이 값이 <code>true</code>이면, 오래된 것처럼 보이지 않더라도 데이터를 다시 로드해요</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">작업을 취소하기 위한 선택적 AbortSignal</p>
  </li>
</ul>