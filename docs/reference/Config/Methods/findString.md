# findString

Find the value of a string config variable as an owned string.

All config files will be looked into, in the order of their
defined level. A higher level means a higher priority. The
first occurrence of the variable will be returned here.

## Signature

```ts
class Config {
  findString(name: string): string | null;
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
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The value of a string config variable.</p>
  </li>
</ul>