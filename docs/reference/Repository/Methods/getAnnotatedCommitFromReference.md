# getAnnotatedCommitFromReference

Creates an Annotated Commit from the given reference.

## Signature

```ts
class Repository {
  getAnnotatedCommitFromReference(reference: Reference): AnnotatedCommit;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reference</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Reference</span>
    <br>
    <p class="param-description">Reference to create an Annotated Commit from.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">An Annotated Commit created from reference.</p>
  </li>
</ul>