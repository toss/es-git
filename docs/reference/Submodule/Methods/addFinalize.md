# addFinalize

Resolve the setup of a new git submodule.

This should be called on a submodule once you have called add setup and
done the clone of the submodule. This adds the `.gitmodules` file and the
newly cloned submodule to the index to be ready to be committed (but
doesn't actually do the commit).

## Signature

```ts
class Submodule {
  addFinalize(): void;
}
```