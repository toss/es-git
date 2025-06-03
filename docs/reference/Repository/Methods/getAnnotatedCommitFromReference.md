# getAnnotatedCommitFromReference

Creates a Annotated Commit from the given reference.

## Signature

```ts
class Repository {
  getAnnotatedCommitFromReference(reference: Reference): AnnotatedCommit;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">reference</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">GitReference</span>
    <br>
    <p class="param-description">Reference to creates a Annotated Commit.</p>
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