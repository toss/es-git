[es-git](../globals.md) / IndexAddAllOptions

# Interface: IndexAddAllOptions

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="force"></a> `force?` | `boolean` | Files that are ignored will be skipped (unlike `addPath`). If a file is already tracked in the index, then it will be updated even if it is ignored. Pass the `force` flag to skip the checking of ignore rules. |
| <a id="disablepathspecmatch"></a> `disablePathspecMatch?` | `boolean` | The `pathspecs` are a list of file names or shell glob patterns that will matched against files in the repository's working directory. Each file that matches will be added to the index (either updating an existing entry or adding a new entry). You can disable glob expansion and force exact matching with the `disablePathspecMatch` flag. |
| <a id="checkpathspec"></a> `checkPathspec?` | `boolean` | To emulate `git add -A` and generate an error if the pathspec contains the exact path of an ignored file (when not using `force`), add the `checkPathspec` flag. This checks that each entry in `pathspecs` that is an exact match to a filename on disk is either not ignored or already in the index. If this check fails, the function will return an error. |
| <a id="onmatch"></a> `onMatch?` | (`args`: [`IndexOnMatchCallbackArgs`](IndexOnMatchCallbackArgs.md)) => `number` | If you provide a callback function, it will be invoked on each matching item in the working directory immediately before it is added to / updated in the index. Returning zero will add the item to the index, greater than zero will skip the item, and less than zero will abort the scan an return an error to the caller. |
