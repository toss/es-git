# extractSignature

Extract a signature from an object identified by its ID.

This method can be used for any object that may be signed, such as commits or tags.

## Signature

```ts
class Repository {
  extractSignature(oid: string): ExtractedSignature | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Object ID (SHA1) of the signed object to extract the signature from.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ExtractedSignature</span>
    <br>
    <p class="param-description">An ExtractedSignature object containing the signature and signed data if the object is signed,<br>         or null if the object is not signed.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">GPG signature of the commit, or null if the commit is not signed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">signedData</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Signed data of the commit.</p>
      </li>
    </ul>
  </li>
</ul>