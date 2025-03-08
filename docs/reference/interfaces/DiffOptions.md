[es-git](../globals.md) / DiffOptions

# Interface: DiffOptions

Structure describing options about how the diff should be executed.

## Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="reverse"></a> `reverse?` | `boolean` | Flag indicating whether the sides of the diff will be reversed. |
| <a id="includeignored"></a> `includeIgnored?` | `boolean` | Flag indicating whether ignored files are included. |
| <a id="recurseignoreddirs"></a> `recurseIgnoredDirs?` | `boolean` | Flag indicating whether ignored directories are traversed deeply or not. |
| <a id="includeuntracked"></a> `includeUntracked?` | `boolean` | Flag indicating whether untracked files are in the diff |
| <a id="recurseuntrackeddirs"></a> `recurseUntrackedDirs?` | `boolean` | Flag indicating whether untracked directories are traversed deeply or not. |
| <a id="includeunmodified"></a> `includeUnmodified?` | `boolean` | Flag indicating whether unmodified files are in the diff. |
| <a id="includetypechange"></a> `includeTypechange?` | `boolean` | If enabled, then Typechange delta records are generated. |
| <a id="includetypechangetrees"></a> `includeTypechangeTrees?` | `boolean` | Event with `includeTypechange`, the tree returned generally shows a deleted blob. This flag correctly labels the tree transitions as a typechange record with the `newFile`'s mode set to tree. Note that the tree SHA will not be available. |
| <a id="ignorefilemode"></a> `ignoreFilemode?` | `boolean` | Flag indicating whether file mode changes are ignored. |
| <a id="ignoresubmodules"></a> `ignoreSubmodules?` | `boolean` | Flag indicating whether all submodules should be treated as unmodified. |
| <a id="ignorecase"></a> `ignoreCase?` | `boolean` | Flag indicating whether case insensitive filenames should be used. |
| <a id="disablepathspecmatch"></a> `disablePathspecMatch?` | `boolean` | If pathspecs are specified, this flag means that they should be applied as an exact match instead of a fnmatch pattern. |
| <a id="skipbinarycheck"></a> `skipBinaryCheck?` | `boolean` | Disable updating the `binary` flag in delta records. This is useful when iterating over a diff if you don't need hunk and data callbacks and want to avoid having to load a file completely. |
| <a id="enablefastuntrackeddirs"></a> `enableFastUntrackedDirs?` | `boolean` | When diff finds an untracked directory, to match the behavior of core Git, it scans the contents for ignored and untracked files. If all contents are ignored, then the directory is ignored; if any contents are not ignored, then the directory is untracked. This is extra work that may not matter in many cases. This flag turns off that scan and immediately labels an untracked directory as untracked (changing the behavior to not match core git). |
| <a id="updateindex"></a> `updateIndex?` | `boolean` | When diff finds a file in the working directory with stat information different from the index, but the OID ends up being the same, write the correct stat information into the index. Note: without this flag, diff will always leave the index untouched. |
| <a id="includeunreadable"></a> `includeUnreadable?` | `boolean` | Include unreadable files in the diff |
| <a id="includeunreadableasuntracked"></a> `includeUnreadableAsUntracked?` | `boolean` | Include unreadable files in the diff as untracked files |
| <a id="forcetext"></a> `forceText?` | `boolean` | Treat all files as text, disabling binary attributes and detection. |
| <a id="forcebinary"></a> `forceBinary?` | `boolean` | Treat all files as binary, disabling text diffs |
| <a id="ignorewhitespace"></a> `ignoreWhitespace?` | `boolean` | Ignore all whitespace |
| <a id="ignorewhitespacechange"></a> `ignoreWhitespaceChange?` | `boolean` | Ignore changes in the amount of whitespace |
| <a id="ignorewhitespaceeol"></a> `ignoreWhitespaceEol?` | `boolean` | Ignore whitespace at the end of line |
| <a id="ignoreblanklines"></a> `ignoreBlankLines?` | `boolean` | Ignore blank lines |
| <a id="showuntrackedcontent"></a> `showUntrackedContent?` | `boolean` | When generating patch text, include the content of untracked files. This automatically turns on `includeUntracked` but it does not turn on `recurseUntrackedDirs`. Add that flag if you want the content of every single untracked file. |
| <a id="showunmodified"></a> `showUnmodified?` | `boolean` | When generating output, include the names of unmodified files if they are included in the `Diff`. Normally these are skipped in the formats that list files (e.g. name-only, name-status, raw). Even with this these will not be included in the patch format. |
| <a id="patience"></a> `patience?` | `boolean` | Use the "patience diff" algorithm |
| <a id="minimal"></a> `minimal?` | `boolean` | Take extra time to find the minimal diff |
| <a id="showbinary"></a> `showBinary?` | `boolean` | Include the necessary deflate/delta information so that `git-apply` can apply given diff information to binary files. |
| <a id="indentheuristic"></a> `indentHeuristic?` | `boolean` | Use a heuristic that takes indentation and whitespace into account which generally can produce better diffs when dealing with ambiguous diff hunks. |
| <a id="contextlines"></a> `contextLines?` | `number` | Set the number of unchanged lines that define the boundary of a hunk (and to display before and after). The default value for this is 3. |
| <a id="interhunklines"></a> `interhunkLines?` | `number` | Set the maximum number of unchanged lines between hunk boundaries before the hunks will be merged into one. The default value for this is 0. |
| <a id="idabbrev"></a> `idAbbrev?` | `number` | The default value for this is `core.abbrev` or 7 if unset. |
| <a id="maxsize"></a> `maxSize?` | `number` | Maximum size (in bytes) above which a blob will be marked as binary automatically. A negative value will disable this entirely. The default value for this is 512MB. |
| <a id="oldprefix"></a> `oldPrefix?` | `string` | The virtual "directory" to prefix old file names with in hunk headers. The default value for this is "a". |
| <a id="newprefix"></a> `newPrefix?` | `string` | The virtual "directory" to prefix new file names with in hunk headers. The default value for this is "b". |
| <a id="pathspecs"></a> `pathspecs?` | `string`[] | Add to the array of paths/fnmatch patterns to constrain the diff. |
