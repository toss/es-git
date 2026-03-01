# iter

Create an iterator over the entries in the reflog.

## Signature

```ts
class Reflog {
  iter(): ReflogIter;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">ReflogIter</span>
    <br>
    <p class="param-description">Iterator over the reflog entries.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the reflog cannot be accessed.</p>
  </li>
</ul>