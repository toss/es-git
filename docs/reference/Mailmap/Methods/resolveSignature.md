# resolveSignature

Resolve a signature to its canonical form using a mailmap.

Returns a new signature with the canonical name and email.

## Signature

```ts
class Mailmap {
  resolveSignature(signature: SignaturePayload): Signature;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">SignaturePayload</span>
    <br>
    <p class="param-description">Signature to resolve</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Email on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Name on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">offset</span><span class="param-type">number</span>
            <br>
            <p class="param-description">Timezone offset, in minutes</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">Time in seconds, from epoch</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description">The resolved signature with canonical name and email</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Email on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Name on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">Time in seconds, from epoch</p>
      </li>
    </ul>
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