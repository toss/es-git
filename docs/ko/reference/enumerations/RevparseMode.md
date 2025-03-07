[es-git](../globals.md) / RevparseMode

# 열거형: RevparseMode

Flags for the Revspec.

## 포함된 값

| 포함된 값 | 값 | 설명 |
| ------ | ------ | ------ |
| <a id="single"></a> `Single` | `1` | The spec targeted a single object (1 << 0) |
| <a id="range"></a> `Range` | `2` | The spec targeted a range of commits (1 << 1) |
| <a id="mergebase"></a> `MergeBase` | `4` | The spec used the `...` operator, which invokes special semantics. (1 << 2) |
