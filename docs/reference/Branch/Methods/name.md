# name

Return the name of the given local or remote branch.

## Signature

```ts
class Branch {
  name(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the given local or remote branch.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the name is not valid utf-8.</p>
  </li>
</ul>