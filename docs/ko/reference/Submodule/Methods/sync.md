# sync

서브모듈 원격 정보를 서브모듈 리포지토리에 복사해요.

이 작업은 서브모듈 URL에 대한 정보를 체크아웃된 서브모듈 설정으로 복사하며, "git submodule sync"처럼 동작해요. 이 기능은 서브모듈의 URL을 변경했거나(또는 업스트림 변경 사항을 가져오는 과정에서 변경되었을 수도 있어요) 로컬 리포지토리를 업데이트해야 할 때 유용해요.

## 시그니처

```ts
class Submodule {
  sync(signal?: AbortSignal | null | undefined): Promise<void>;
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