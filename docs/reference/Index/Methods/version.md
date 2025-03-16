# version

Get index on-disk version.

## Signature

```ts
class Index {
  version(): number;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">number</span>
    <br>
    <p class="param-description">Index on-disk version.<br>Valid return values are 2, 3, or 4. If 3 is returned, an index<br>with version 2 may be written instead, if the extension data in<br>version 3 is not necessary.</p>
  </li>
</ul>