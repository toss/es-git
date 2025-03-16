# resolve

Resolve a symbolic reference to a direct reference.

This method iteratively peels a symbolic reference until it resolves to
a direct reference to an OID.

If a direct reference is passed as an argument, a copy of that
reference is returned.

## Signature

```ts
class Reference {
  resolve(): Reference;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reference</span>
    <br>
    <p class="param-description">Resolved reference.</p>
  </li>
</ul>