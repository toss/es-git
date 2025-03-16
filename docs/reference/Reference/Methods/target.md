# target

Get the OID pointed to by a direct reference.

Only available if the reference is direct (i.e. an object id reference,
not a symbolic one).

## Signature

```ts
class Reference {
  target(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">OID pointed to by a direct reference.</p>
  </li>
</ul>