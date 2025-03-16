# prune

Prune tracking refs that are no longer present on remote.

## Signature

```ts
class Remote {
  prune(options?: PruneOptions, signal?: AbortSignal): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | PruneOptions</span>
    <br>
    <p class="param-description">Options for prune remote.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">credential</span><span class="param-type">Credential</span>
        <br>
        <p class="param-description">A interface to represent git credentials in libgit2.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>