# openConfig

Create a new config instance containing a single on-disk file

## Signature

```ts
function openConfig(path: string): Config;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Path to config file.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">Config instance representing a git configuration key/value store.</p>
  </li>
</ul>