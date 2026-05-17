# getBool

Get the value of a boolean config variable.

All config files will be looked into, in the order of their defined
level. A higher level means a higher priority. The first occurrence of
the variable will be returned here.

## Signature

```ts
class Config {
  getBool(name: string): boolean;
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
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">The value of a boolean config variable.</p>
  </li>
</ul>