# remove

Delete a config variable from the config file with the highest level
(usually the local one).

## Signature

```ts
class Config {
  remove(name: string): void;
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