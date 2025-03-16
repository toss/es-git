# simplifyFirstParent

Simplify the history by first-parent.

No parents other than the first for each commit will be enqueued.

## 시그니처

```ts
class Revwalk {
  simplifyFirstParent(): this;
}
```