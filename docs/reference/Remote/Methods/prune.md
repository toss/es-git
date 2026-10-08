# prune

Prune tracking refs that are no longer present on remote.

## Signature

```ts
class Remote {
  prune(options?: PruneOptions | null | undefined, signal?: AbortSignal | null | undefined): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">PruneOptions | null</span>
    <br>
    <p class="param-description">Options for prune remote.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">credential</span><span class="param-type">Credential | ((args: CredentialCallbackArgs) =&gt; Credential | Promise&lt;Credential | null | undefined&gt; | null | undefined)</span>
        <br>
        <p class="param-description">Credential to authenticate with, or a function that returns one.  Pruning compares against the references from the last connection to the remote and does not connect again, so this is currently never used.</p>
        <p class="param-description">A interface to represent git credentials in libgit2.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">AbortSignal | null</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>