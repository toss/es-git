[es-git](../globals.md) / IndexUpdateAllOptions

# 인터페이스: IndexUpdateAllOptions

## 속성

| 속성 | 유형 | 설명 |
| ------ | ------ | ------ |
| <a id="onmatch"></a> `onMatch?` | (`args`: [`IndexOnMatchCallbackArgs`](IndexOnMatchCallbackArgs.md)) => `number` | If you provide a callback function, it will be invoked on each matching item in the index immediately before it is updated (either refreshed or removed depending on working directory state). Return 0 to proceed with updating the item, > 0 to skip the item, and < 0 to abort the scan. |
