# describe

Describes a commit

Performs a describe operation on the current commit and the worktree.
After performing a describe on `HEAD`, a status is run and description is
considered to be dirty if there are.

## Signature

```ts
class Repository {
  describe(options?: DescribeOptions | null | undefined): Describe;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | DescribeOptions</span>
    <br>
    <p class="param-description">Options for describe operation.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">describeAll</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Sets the reference lookup strategy  This behaves like the <code>--all</code> option to git-describe.</p>
      </li>
      <li class="param-li">
        <span class="param-name">describeTags</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Sets the reference lookup strategy  This behaves like the <code>--tags</code> option to git-describe.</p>
      </li>
      <li class="param-li">
        <span class="param-name">maxCandidatesTags</span><span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">onlyFollowFirstParent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Indicates when calculating the distance from the matching tag or reference whether to only walk down the first-parent ancestry.</p>
      </li>
      <li class="param-li">
        <span class="param-name">pattern</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">showCommitOidAsFallback</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If no matching tag or reference is found whether a describe option would normally fail. This option indicates, however, that it will instead fall back to showing the full id of the commit.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Describe</span>
    <br>
    <p class="param-description">Instance of describe.</p>
  </li>
</ul>