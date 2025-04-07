# asCommit

Attempt to view this object as a commit.

## Signature

```ts
class GitObject {
  asCommit(): Commit | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Commit</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if the object is not actually a commit.</p>
  </li>
</ul>