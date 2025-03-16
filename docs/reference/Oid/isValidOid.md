# isValidOid

Check if given string is valid Oid.

## Signature

```ts
function isValidOid(value: string): boolean;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">String to check if is valid Oid.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>false</code>  if the string is empty, is longer than 40 hex<br>characters, or contains any non-hex characters.</p>
  </li>
</ul>