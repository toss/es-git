# targetPeel

Return the peeled OID target of this reference.

This peeled OID only applies to direct references that point to a hard.

## Signature

```ts
class Reference {
  targetPeel(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Peeled OID of this reference.</p>
  </li>
</ul>