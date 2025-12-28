# ignoreRule

Get the ignore rule that will be used for the submodule.

## Signature

```ts
class Submodule {
  ignoreRule(): SubmoduleIgnore;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">SubmoduleIgnore</span>
    <br>
    <p class="param-description">The ignore rule that will be used for the submodule.</p>
    <p class="param-description">Submodule ignore values<br><br>These values represent settings for the <code>submodule.$name.ignore</code><br>configuration value which says how deeply to look at the working<br>directory when getting the submodule status.</p>
  </li>
</ul>