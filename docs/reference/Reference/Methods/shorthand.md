# shorthand

Get the full shorthand of a reference.

This will transform the reference name into a name "human-readable"
version. If no shortname is appropriate, it will return the full name.

## Signature

```ts
class Reference {
  shorthand(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Full shorthand of a reference.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the shorthand is not valid utf-8.</p>
  </li>
</ul>