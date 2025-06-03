# createRemote

Add a remote with the default fetch refspec to the repository’s configuration.

## Signature

```ts
class Repository {
  createRemote(name: string, url: string, options?: CreateRemoteOptions | null | undefined): Remote;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the remote.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">url</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Remote url.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateRemoteOptions</span>
    <br>
    <p class="param-description">Options for creating remote.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">fetchRefspec</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Remote</span>
    <br>
    <p class="param-description">Created remote.</p>
  </li>
</ul>