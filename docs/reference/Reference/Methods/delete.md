# delete

Delete an existing reference.

This method works for both direct and symbolic references. The reference
will be immediately removed on disk.

## Signature

```ts
class Reference {
  delete(): void;
}
```

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">This method will throws an error if the reference has changed from the<br>time it was looked up.</p>
  </li>
</ul>