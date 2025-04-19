# entries

Iterate over all the config variables.

## Signature

```ts
class Config {
  entries(glob?: string): ConfigEntries;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">glob</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">If <code>glob</code> is provided, then the iterator will only iterate over all variables whose name matches the pattern. The regular expression is applied case-sensitively on the normalized form of the variable name: the section and variable parts are lower-cased. The subsection is left unchanged.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ConfigEntries</span>
    <br>
    <p class="param-description">An iterator over the  <code>ConfigEntry</code>  values of a config.</p>
  </li>
</ul>

## Examples

```ts
import { openDefaultConfig } from 'es-git';

const config = openDefaultConfig();
for (const entry of config.entries()) {
  console.log(entry.name, entry.value);
}
```