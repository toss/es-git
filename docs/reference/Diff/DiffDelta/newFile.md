# newFile

Return the file which represents the "to" side of the diff.

What side this means depends on the function that was used to generate
the diff and will be documented on the function itself.

## Signature

```ts
class DiffDelta {
  newFile(): DiffFile;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DiffFile</span>
    <br>
    <p class="param-description">The file which represents the &quot;to&quot; side of the diff.</p>
  </li>
</ul>