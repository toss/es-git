# getAnnotatedCommitFromFetchHead

Creates a Annotated Commit from `FETCH_HEAD`.

## Signature

```ts
class Repository {
  getAnnotatedCommitFromFetchHead(
    branchName: string,
    remoteUrl: string,
    id: string,
  ): AnnotatedCommit;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">branchName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name of the remote branch.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">remoteUrl</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Url of the remote.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The commit object id of the remote branch.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">AnnotatedCommit</span>
    <br>
    <p class="param-description">An Annotated Commit created from  <code>FETCH_HEAD</code> .</p>
  </li>
</ul>