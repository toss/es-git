# writeTree

인덱스를 트리 형태로 기록해요.

이 메소드는 인덱스를 검색하여 현재 상태를 디스크에 기록해요; 인덱스에 저장된 각 서브트리에 대해 재귀적으로 트리 개체를 생성하지만, 최종적으로는 루트 트리의 OID만 반환해요. 이 OID는 예를 들어 커밋을 생성할 때 사용될 수 있어요.

인덱스 인스턴스는 bare이면 안 되며, 기존 리포지토리와 연결되어 있어야 해요.

인덱스에는 충돌 상태인 파일이 없어야 해요.

## 시그니처

```ts
class Index {
  writeTree(): string;
}
```
