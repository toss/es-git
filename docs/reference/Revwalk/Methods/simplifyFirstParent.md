# simplifyFirstParent

Simplify the history by first-parent.

No parents other than the first for each commit will be enqueued.

## Signature

```ts
class Revwalk {
  simplifyFirstParent(): this;
}
```