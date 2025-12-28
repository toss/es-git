# workdirId

Get the OID for the submodule in the current working directory.

This returns the OID that corresponds to looking up `HEAD` in the
checked out submodule. If there are pending changes in the index or
anything else, this won't notice that.

## Signature

```ts
class Submodule {
  workdirId(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">The OID for the submodule in the current working directory.</p>
  </li>
</ul>