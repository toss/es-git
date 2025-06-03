# setHeadDetachedFromAnnotated

Make the repository HEAD directly point to the commit.

If the provided commitish cannot be found in the repository, the HEAD
is unaltered and an error is returned.
If the provided commitish cannot be peeled into a commit, the HEAD is
unaltered and an error is returned.
Otherwise, the HEAD will eventually be detached and will directly point
to the peeled commit.

## Signature

```ts
class Repository {
  setHeadDetachedFromAnnotated(commitish: AnnotatedCommit): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitish</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">An Annotated Commit which the HEAD should point to.</p>
  </li>
</ul>