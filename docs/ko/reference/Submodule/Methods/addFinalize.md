# addFinalize

새 git 서브모듈의 설정을 해결해요.

이 메서드는 add setup을 호출하고 서브모듈의 클론을 완료한 다음, 해당 서브모듈에서 호출해야 해요. 이 메서드는 `.gitmodules` 파일과 새로 클론된 서브모듈을 인덱스에 추가해서 커밋할 준비가 되도록 해요(하지만 실제로 커밋을 수행하지는 않아요).

## 시그니처

```ts
class Submodule {
  addFinalize(): void;
}
```