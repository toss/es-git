# hideHead

리포지토리의 `HEAD`를 숨겨요.

이 메서드를 호출하면 `HEAD`가 가리키는 커밋과 그 조상 커밋들이 Revwalk 결과에서 제외돼요.

## 시그니처

```ts
class Revwalk {
  hideHead(): this;
}
```