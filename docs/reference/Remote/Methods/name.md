# name

Get the remote's name.

## Signature

```ts
class Remote {
  name(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if this remote has not yet been named.</p>
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