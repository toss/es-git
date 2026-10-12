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
        <span class="param-name">credential</span><span class="param-type">Credential</span>
        <br>
        <p class="param-description">An interface to represent git credentials in libgit2.<br><br><code>SSHKeyFromPath</code> requires <code>privateKeyPath</code>, <code>SSHKey</code> requires <code>privateKey</code>, and<br><code>Plain</code> requires <code>password</code> (an empty password is allowed). The username defaults<br>to <code>&quot;git&quot;</code>. Public keys and passphrases are optional.<br><br>Credentials are validated before connecting, even for public or local remotes<br>that do not require authentication. Omit <code>credential</code> when authentication is not needed.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">AbortSignal | null</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws an  <code>InvalidArg</code>  error if  <code>options.credential</code>  is missing a field required by its  <code>type</code> <br>( <code>privateKeyPath</code>  for  <code>SSHKeyFromPath</code> ,  <code>privateKey</code>  for  <code>SSHKey</code> ,  <code>password</code>  for  <code>Plain</code> ),<br>even if the remote does not require authentication. Throws an error if the prune fails.</p>
  </li>
</ul>