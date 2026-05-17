# findSubmodule

Lookup submodule information by name or path.

Given either the submodule name or path (they are usually the same),
this returns a structure describing the submodule.

## Signature

```ts
class Repository {
  findSubmodule(name: string): Submodule | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of or path to the submodule; trailing slashes okay.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Submodule | null</span>
    <br>
    <p class="param-description">The submodule. Returns  <code>null</code>  if the submodule is not found.</p>
  </li>
</ul>