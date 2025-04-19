# multivar

Iterate over the values of a multivar.

## Signature

```ts
class Config {
  multivar(name: string, regexp?: string): ConfigEntries;
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
    <span class="param-name">regexp</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">If <code>regexp</code> is provided, then the iterator will only iterate over all values which match the pattern. The regular expression is applied case-sensitively on the normalized form of the variable name: the section and variable parts are lower-cased. The subsection is left unchanged.</p>
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