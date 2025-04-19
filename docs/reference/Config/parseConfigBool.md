# parseConfigBool

Parse a string as a bool.

## Signature

```ts
function parseConfigBool(value: string): boolean;
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
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Interprets &quot;true&quot;, &quot;yes&quot;, &quot;on&quot;, 1, or any non-zero number as  <code>true</code> .<br>Interprets &quot;false&quot;, &quot;no&quot;, &quot;off&quot;, 0, or an empty string as  <code>false</code> .</p>
  </li>
</ul>