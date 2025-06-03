# statuses

파일 상태 정보를 수집하고 반환된 구조체를 채워요.

상태를 필터링하기 위해 옵션에 pathspec이 주어진 경우, 이름 변경 감지(활성화한 경우)의 결과가 정확하지 않을 수 있다는 점에 주의해요. 이름 변경 감지를 올바르게 수행하려면 모든 파일을 고려할 수 있도록 pathspec 없이 호출해야 해요.

## 시그니처

```ts
class Repository {
  statuses(): Statuses;
}
```

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Statuses</span>
    <br>
    <p class="param-description">리포지토리에 대한 상태 정보 목록의 컨테이너</p>
  </li>
</ul>