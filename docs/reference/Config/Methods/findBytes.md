# findBytes

Find the value of a string config variable as a byte slice.

## Signature

```ts
class Config {
  findBytes(name: string): Uint8Array | null;
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
    <span class="param-type">null | Uint8Array&lt;ArrayBufferLike&gt;</span>
    <br>
    <p class="param-description">The value of a string config variable as a byte slice.</p>
  </li>
</ul>