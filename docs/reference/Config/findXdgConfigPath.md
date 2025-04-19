# findXdgConfigPath

Locate the path to the global XDG compatible configuration file.

The XDG compatible configuration file is usually located in
`$HOME/.config/git/config`.

## Signature

```ts
function findXdgConfigPath(): string | null;
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The path to the XDG compatible configuration file.</p>
  </li>
</ul>