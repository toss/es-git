# rename

Rename an existing reference.

This method works for both direct and symbolic references.

## Signature

```ts
class Reference {
  rename(newName: string, options?: RenameReferenceOptions | null | undefined): Reference;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name to rename an existing reference.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | RenameReferenceOptions</span>
    <br>
    <p class="param-description">Options to rename an existing reference.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If the force flag is not enabled, and there&#39;s already a reference with the given name, the renaming will fail.</p>
      </li>
      <li class="param-li">
        <span class="param-name">logMessage</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reference</span>
    <br>
    <p class="param-description">Renamed reference.</p>
  </li>
</ul>
