# getBytes

Get the value of a string config variable as a byte slice.

## Signature

```ts
class Config {
  getBytes(name: string): Uint8Array;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of config entry.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Uint8Array</span>
    <br>
    <p class="param-description">The value of a string config variable as a byte slice.</p>
  </li>
</ul>