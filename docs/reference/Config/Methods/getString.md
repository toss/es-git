# getString

Get the value of a string config variable as an owned string.

All config files will be looked into, in the order of their
defined level. A higher level means a higher priority. The
first occurrence of the variable will be returned here.

## Signature

```ts
class Config {
  getString(name: string): string;
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
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The value of a string config variable.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">An error will be returned if the config value is not valid utf-8.</p>
  </li>
</ul>