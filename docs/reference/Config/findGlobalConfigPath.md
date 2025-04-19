# findGlobalConfigPath

Locate the path to the global configuration file.

The user or global configuration file is usually located in
`$HOME/.gitconfig`.

This method will try to guess the full path to that file, if the file
exists. The returned path may be used on any method call to load
the global configuration file.

This method will not guess the path to the XDG compatible config file
(`.config/git/config`).

## Signature

```ts
function findGlobalConfigPath(): string | null;
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The path to the global configuration file.</p>
  </li>
</ul>