# inmemoryIndex

Gets the index produced by the last operation, which is the result of
`next()` and which will be committed by the next invocation of
`commit()`. This is useful for resolving conflicts in an in-memory
rebase before committing them.

This is only applicable for in-memory rebases; for rebases within a
working directory, the changes were applied to the repository's index.

## Signature

```ts
class Rebase {
  inmemoryIndex(): Index;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">The index produced by the last operation.</p>
  </li>
</ul>