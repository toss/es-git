# setBool

Set the value of a boolean config variable in the config file with the
highest level (usually the local one).

## Signature

```ts
class Config {
  setBool(name: string, value: boolean): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of config entry.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">value</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
    <br>
    <p class="param-description">The value of config entry.</p>
  </li>
</ul>