# getAnnotatedCommit

Creates an Annotated Commit from the given commit.

## Signature

```ts
class Repository {
  getAnnotatedCommit(commit: Commit): AnnotatedCommit;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commit</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Commit</span>
    <br>
    <p class="param-description">Commit to creates a Annotated Commit.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">An Annotated Commit created from commit.</p>
  </li>
</ul>