# name

Get the filename of a tree entry.

## Signature

```ts
class TreeEntry {
  name(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The filename of a tree entry.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the name is not valid utf-8.</p>
  </li>
</ul>