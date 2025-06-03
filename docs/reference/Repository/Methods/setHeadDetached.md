# setHeadDetached

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
  setHeadDetached(commitish: Commit): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commit</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
  </li>
</ul>