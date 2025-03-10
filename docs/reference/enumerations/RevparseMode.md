[es-git](../globals.md) / RevparseMode

# Enumeration: RevparseMode

Flags for the revparse.

## Enumeration Members

| Enumeration Member | Value | Description |
| ------ | ------ | ------ |
| <a id="single"></a> `Single` | `1` | The spec targeted a single object |
| <a id="range"></a> `Range` | `2` | The spec targeted a range of commits |
| <a id="mergebase"></a> `MergeBase` | `4` | The spec used the `...` operator, which invokes special semantics. |
