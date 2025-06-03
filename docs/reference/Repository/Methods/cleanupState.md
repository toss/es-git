# cleanupState

Remove all the metadata associated with an ongoing command like merge,
revert, cherry-pick, etc. For example: `MERGE_HEAD`, `MERGE_MSG`, etc.

## Signature

```ts
class Repository {
  cleanupState(): void;
}
```