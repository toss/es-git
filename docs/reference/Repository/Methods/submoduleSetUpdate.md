# submoduleSetUpdate

Set the update rule for the submodule in the configuration

This setting won't affect any existing instances.

## Signature

```ts
class Repository {
  submoduleSetUpdate(name: string, update: SubmoduleUpdate): void;
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
    <span class="param-name">update</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">SubmoduleUpdate</span>
    <br>
    <p class="param-description">The new value to use.</p>
    <p class="param-description">Submodule update values<br><br>These values represent settings for the <code>submodule.$name.update</code><br>configuration value which says how to handle <code>git submodule update</code><br>for this submodule. The value is usually set in the &quot;.gitmodules&quot;<br>file and copied to &quot;.git/config&quot; when the submodule is initialized.</p>
  </li>
</ul>