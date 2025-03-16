# createSignature

Create a new action signature.

## Signature

```ts
function createSignature(
  name: string,
  email: string,
  timeOptions?: SignatureTimeOptions,
): Signature;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name on the signature.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Email on the signature.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">timeOptions</span><span class="param-type">null | SignatureTimeOptions</span>
    <br>
    <p class="param-description">Time options for signature.</p>
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

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description"></p>
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

## Examples

```ts
import { createSignature } from 'es-git';

const author = createSignature(
  'Seokju Na',
  'seokju.me@toss.im',
);
```