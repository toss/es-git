# id

Returns the Oid of this item.

If this entry represents an absent side of a diff (e.g. the `oldFile`
of a `Added` delta), then the oid returned will be zeroes.

## Signature

```ts
class DiffFile {
  id(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The Oid of this item.</p>
  </li>
</ul>