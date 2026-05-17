# refname

Get the refname that the given Annotated Commit refers to.

## Signature

```ts
class AnnotatedCommit {
  refname(): string | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string | null</span>
    <br>
    <p class="param-description">The refname that this Annotated Commit refers to. If this created from a reference,<br>the return value is  <code>null</code> .</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the refname is not valid utf-8.</p>
  </li>
</ul>