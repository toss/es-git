# addEntry

Add a new Mailmap entry.

Maps an author/committer (specified by `replace_name` and `replace_email`)
to the specified real name and email. The `replace_email` is required but
the other parameters can be null.

If both `replace_name` and `replace_email` are provided, then the entry will
apply to those who match both. If only `replace_name` is provided,
it will apply to anyone with that name, regardless of email. If only
`replace_email` is provided, it will apply to anyone with that email,
regardless of name.

## Signature

```ts
class Mailmap {
  addEntry(entry: AddMailmapEntryData): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">entry</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">AddMailmapEntryData</span>
    <br>
    <p class="param-description">The mailmap entry data.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">realEmail</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">realName</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">replaceEmail</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">replaceName</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">void</span>
    <br>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">An error if the operation failed.</p>
  </li>
</ul>