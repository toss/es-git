# updateStrategy

Get the update rule that will be used for the submodule.

## Signature

```ts
class Submodule {
  updateStrategy(): SubmoduleUpdate;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">SubmoduleUpdate</span>
    <br>
    <p class="param-description">The update rule that will be used for the submodule.</p>
    <p class="param-description">Submodule update values<br><br>These values represent settings for the <code>submodule.$name.update</code><br>configuration value which says how to handle <code>git submodule update</code><br>for this submodule. The value is usually set in the &quot;.gitmodules&quot;<br>file and copied to &quot;.git/config&quot; when the submodule is initialized.</p>
  </li>
</ul>