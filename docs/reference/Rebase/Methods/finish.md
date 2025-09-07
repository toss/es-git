# finish

Finishes a rebase that is currently in progress once all patches have
been applied.

## Signature

```ts
class Rebase {
  finish(signature?: SignaturePayload | undefined | null): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-type">null | SignaturePayload</span>
    <br>
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