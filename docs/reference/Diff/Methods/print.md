# print

Iterate over a diff generating formatted text output.

The output follows `git diff`. For the `Patch` and `PatchId` formats, each content line is prefixed with its origin: `+` for an added line, `-` for a deleted line and a single space for a context line. File headers, hunk headers (`@@ ... @@`), binary notices and `\ No newline at end of file` markers are emitted as-is.

## Signature

```ts
class Diff {
  print(options?: DiffPrintOptions | null | undefined): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DiffPrintOptions</span>
    <br>
    <p class="param-description">Print options for diff.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">format</span><span class="param-type">DiffFormat</span>
        <br>
        <p class="param-description">Possible output formats for diff data.<br><br>- <code>Patch</code>: Full <code>git diff</code> (default)<br>- <code>PatchHeader</code> : Just the headers of the patch<br>- <code>Raw</code> : Like <code>git diff --raw</code> the headers of the patch<br>- <code>NameOnly</code> : Like <code>git diff --name-only</code><br>- <code>NameStatus</code> : Like <code>git diff --name-status</code><br>- <code>PatchId</code> : <code>git diff</code> as used by <code>git patch-id</code></p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Formatted text output. With the default <code>Patch</code> format, this is a unified patch like <code>git diff</code>.</p>
  </li>
</ul>

### Examples

```ts
const diff = repo.diffIndexToWorkdir();
console.log(diff.print());
// diff --git a/A b/A
// index f70f10e..784f93d 100644
// --- a/A
// +++ b/A
// @@ -1 +1 @@
// -A
// +A modified
```
