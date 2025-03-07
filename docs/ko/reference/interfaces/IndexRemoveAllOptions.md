[es-git](../globals.md) / IndexRemoveAllOptions

# 인터페이스: IndexRemoveAllOptions

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="onmatch"></a> `onMatch?` | (`args`: [`IndexOnMatchCallbackArgs`](IndexOnMatchCallbackArgs.md)) => `number` | If you provide a callback function, it will be invoked on each matching item in the index immediately before it is removed. Return 0 to remove the item, > 0 to skip the item, and < 0 to abort the scan. |
