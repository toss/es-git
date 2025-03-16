# reset

Reset a revwalk to allow re-configuring it.

The revwalk is automatically reset when iteration of its commits
completes.

## 시그니처

```ts
class Revwalk {
  reset(): this;
}
```