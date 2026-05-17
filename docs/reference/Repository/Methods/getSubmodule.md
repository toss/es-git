# getSubmodule

Lookup submodule information by name or path.

Given either the submodule name or path (they are usually the same),
this returns a structure describing the submodule.

## Signature

```ts
class Repository {
  getSubmodule(name: string): Submodule;
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
    <span class="param-type">Submodule</span>
    <br>
    <p class="param-description">The submodule.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the submodule not found.</p>
  </li>
</ul>