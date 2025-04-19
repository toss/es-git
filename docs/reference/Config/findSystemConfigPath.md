# findSystemConfigPath

Locate the path to the system configuration file.

If `/etc/gitconfig` doesn't exist, it will look for `%PROGRAMFILES%`.

## Signature

```ts
function findSystemConfigPath(): string | null;
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The path to the system configuration file.</p>
  </li>
</ul>