# getI64

Get the value of an integer config variable.

All config files will be looked into, in the order of their defined
level. A higher level means a higher priority. The first occurrence of
the variable will be returned here.

## Signature

```ts
class Config {
  getI64(name: string): number;
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
    <span class="param-type">number</span>
    <br>
    <p class="param-description">The value of an integer config variable.</p>
  </li>
</ul>