# print

Iterate over a diff generating formatted text output.

## Signature

```ts
class Diff {
  print(options?: DiffPrintOptions | null): string;
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
        <p class="param-description">Possible output formats for diff data.<br><br>- <code>DiffFormat.Patch</code>: Full <code>git diff</code> (default)<br>- <code>DiffFormat.PatchHeader</code> : Just the headers of the patch<br>- <code>DiffFormat.Raw</code> : Like <code>git diff --raw</code> the headers of the patch<br>- <code>DiffFormat.NameOnly</code> : Like <code>git diff --name-only</code><br>- <code>DiffFormat.NameStatus</code> : Like <code>git diff --name-status</code><br>- <code>DiffFormat.PatchId</code> : <code>git diff</code> as used by <code>git patch-id</code></p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Formatted text output.</p>
  </li>
</ul>