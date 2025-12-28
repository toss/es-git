# init

서브모듈 정보를 ".git/config" 파일로 복사해요.

"git submodule init"과 마찬가지로, 서브모듈에 대한 정보를 ".git/config"에 복사해요. 위의 접근자 함수를 사용해서 메모리 내 git_submodule 개체를 변경하고, .gitmodules에 있는 내용을 덮어쓰면서 config에 무엇이 기록될지 제어할 수 있어요.

기본적으로 기존 항목은 덮어쓰지 않지만, `overwrite`에 `true`를 전달하면 강제로 업데이트돼요.

## 시그니처

```ts
class Submodule {
  init(
    overwrite?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">overwrite</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">기본적으로 기존 항목은 덮어쓰지 않지만, 이를 true로 설정하면 강제로 업데이트되도록 하는 옵션</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">작업을 취소하기 위한 선택적 AbortSignal</p>
  </li>
</ul>