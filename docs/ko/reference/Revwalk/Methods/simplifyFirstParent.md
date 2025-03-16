# simplifyFirstParent

첫 번째 부모(First Parent)를 기준으로 히스토리를 단순화해요.

각 커밋당 첫 번째 부모 외의 다른 부모들은 큐에 추가되지 않아요.

## 시그니처

```ts
class Revwalk {
  simplifyFirstParent(): this;
}
```