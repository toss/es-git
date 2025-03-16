# writeTree

Write the index as a tree.

This method will scan the index and write a representation of its
current state back to disk; it recursively creates tree objects for each
of the subtrees stored in the index, but only returns the OID of the
root tree. This is the OID that can be used e.g. to create a commit.

The index instance cannot be bare, and needs to be associated to an
existing repository.

The index must not contain any file in conflict.

## Signature

```ts
class Index {
  writeTree(): void;
}
```