# setHead

Make the repository `HEAD` point to the specified reference.

If the provided reference points to a tree or a blob, the `HEAD` is
unaltered and an error is returned.

If the provided reference points to a branch, the `HEAD` will point to
that branch, staying attached, or become attached if it isn't yet. If
the branch doesn't exist yet, no error will be returned. The `HEAD` will
then be attached to an unborn branch.

Otherwise, the `HEAD` will be detached and will directly point to the
commit.

## Signature

```ts
class Repository {
  setHead(refname: string): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refname</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Specified reference to point into <code>HEAD</code>.</p>
  </li>
</ul>