# openDefaultConfig

Open the global, XDG and system configuration files

Utility wrapper that finds the global, XDG and system configuration
files and opens them into a single prioritized config object that can
be used when accessing default config data outside a repository.

## Signature

```ts
function openDefaultConfig(): Config;
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Config</span>
    <br>
    <p class="param-description">Config instance representing a git configuration key/value store.</p>
  </li>
</ul>