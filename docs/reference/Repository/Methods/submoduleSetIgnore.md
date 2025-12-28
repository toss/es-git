# submoduleSetIgnore

Set the ignore rule for the submodule in the configuration

This does not affect any currently-loaded instances.

## Signature

```ts
class Repository {
  submoduleSetIgnore(name: string, ignore: SubmoduleIgnore): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the submodule.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">ignore</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">The new value for the ignore rule.</p>
    <p class="param-description">Submodule ignore values<br><br>These values represent settings for the <code>submodule.$name.ignore</code><br>configuration value which says how deeply to look at the working<br>directory when getting the submodule status.</p>
  </li>
</ul>