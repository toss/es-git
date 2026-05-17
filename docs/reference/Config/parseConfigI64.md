# parseConfigI64

Parse a string as an i64; handles suffixes like k, M, or G, and
multiplies by the appropriate power of 1024.

## Signature

```ts
function parseConfigI64(value: string): number;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Input value.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">Parsed i64 value.</p>
  </li>
</ul>