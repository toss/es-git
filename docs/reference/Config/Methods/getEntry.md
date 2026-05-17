# getEntry

Get the entry for a config variable.

## Signature

```ts
class Config {
  getEntry(name: string): ConfigEntry;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of config entry.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ConfigEntry</span>
    <br>
    <p class="param-description"><code>ConfigEntry</code>  object representing a certain entry owned by a  <code>Config</code>  instance.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">includeDepth</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">Depth of includes where this variable was found</p>
      </li>
      <li class="param-li">
        <span class="param-name">level</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">ConfigLevel</span>
        <br>
        <p class="param-description">The configuration level of this entry.</p>
        <p class="param-description">- <code>ProgramData</code> : System-wide on Windows, for compatibility with portable git.<br>- <code>System</code> : System-wide configuration file. (e.g. <code>/etc/gitconfig</code>)<br>- <code>XDG</code> : XDG-compatible configuration file. (e.g. <code>~/.config/git/config</code>)<br>- <code>Global</code> : User-specific configuration. (e.g. <code>~/.gitconfig</code>)<br>- <code>Local</code> : Repository specific config. (e.g. <code>$PWD/.git/config</code>)<br>- <code>Worktree</code> : Worktree specific configuration file. (e.g. <code>$GIT_DIR/config.worktree</code>)<br>- <code>App</code> : Application specific configuration file.<br>- <code>Highest</code> : Highest level available.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">The name of this entry.</p>
      </li>
      <li class="param-li">
        <span class="param-name">value</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The value of this entry. If no value is defined, the value will be <code>null</code>.</p>
      </li>
    </ul>
  </li>
</ul>