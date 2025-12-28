# apply

Apply a Diff to the given repo, making changes directly in the working directory, the index, or both.

## Signature

```ts
class Repository {
  apply(diff: Diff, location: ApplyLocation, options?: ApplyOptions | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">The diff to apply</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">location</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">ApplyLocation</span>
    <br>
    <p class="param-description">The location to apply</p>
    <p class="param-description">Possible application locations for git_apply<br>see &lt;https://libgit2.org/libgit2/#HEAD/type/git_apply_options&gt;</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | ApplyOptions</span>
    <br>
    <p class="param-description">The options for the apply</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">check</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Don&#39;t actually make changes, just test that the patch applies.</p>
      </li>
    </ul>
  </li>
</ul>