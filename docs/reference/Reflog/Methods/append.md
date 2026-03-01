# append

Append a new entry to the reflog.

## Signature

```ts
class Reflog {
  append(newOid: string, committer: Signature, msg?: string | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">newOid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">New object ID (SHA1) for this reflog entry.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">committer</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Signature</span>
    <br>
    <p class="param-description">Committer signature for this reflog entry.</p>
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
  <li class="param-li param-li-root">
    <span class="param-name">msg</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">Optional message for this reflog entry.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the OID is invalid or if appending fails.</p>
  </li>
</ul>