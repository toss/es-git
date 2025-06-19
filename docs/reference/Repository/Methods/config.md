# config

Get the configuration file for this repository.

## Signature

```ts
class Repository {
  config(): Config;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">If a configuration file has not been set, the default config set for the<br>repository will be returned, including global and system configurations<br>(if they are available).</p>
  </li>
</ul>