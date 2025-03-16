# write

메모리 내의 기존 인덱스 개체를 원자적 파일 잠금을 사용하여 디스크에 다시 기록해요.

## 시그니처

```ts
class Index {
  write(): void;
}
```