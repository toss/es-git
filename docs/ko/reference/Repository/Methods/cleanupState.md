# cleanupState

병합, 되돌리기, 체리픽 등과 같은 진행 중인 명령과 관련된 모든 메타데이터를 제거해요. 예: `MERGE_HEAD`, `MERGE_MSG` 등.

## 시그니처

```ts
class Repository {
  cleanupState(): void;
}
```