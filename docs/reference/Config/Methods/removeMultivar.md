# removeMultivar

Remove multivar config variables in the config file with the highest level (usually the
local one).

## Signature

```ts
class Config {
  removeMultivar(name: string, regexp: string): void;
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
    <span class="param-name">regexp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The regular expression is applied case-sensitively on the value.</p>
  </li>
</ul>