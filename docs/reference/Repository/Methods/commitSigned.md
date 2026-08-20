# commitSigned

Create a signed commit from externally signed commit content.

This writes the signed commit to the object database but does not update
`HEAD` or any other reference. If `signatureField` is omitted, Git's
default `gpgsig` field is used.

## Signature

```ts
class Repository {
  commitSigned(commitContent: string, signature: string, signatureField?: string | null | undefined): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitContent</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Commit content returned by <code>commitCreateBuffer</code>.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">External signature for the commit content.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signatureField</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">Signature field name. Defaults to <code>gpgsig</code>.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">ID(SHA1) of created commit.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the commit content, signature, or signature field is invalid.</p>
  </li>
</ul>