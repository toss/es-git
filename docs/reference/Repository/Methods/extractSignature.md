# extractSignature

Extract the signature from a signed Git object (commit, tag, etc.).

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
    <span class="param-type">ExtractedSignature | null</span>
    <br>
    <p class="param-description">An ExtractedSignature object containing the signature and signed data if the object is signed, or null if the object is not signed.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-type">string</span>
        <br>
        <p class="param-description">GPG signature of the Git object.</p>
      </li>
      <li class="param-li">
        <span class="param-name">signedData</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Signed data of the Git object.</p>
      </li>
    </ul>
  </li>
</ul>

## Example

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');

// Extract the signature from a commit
const signatureInfo = repo.extractSignature(commit.id());

if (signatureInfo) {
  console.log('Object is signed!');
  console.log('Signature:', signatureInfo.signature);
  console.log('Signed data:', signatureInfo.signedData);
} else {
  console.log('Object is not signed');
}
``` 