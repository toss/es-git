# sync

Copy submodule remote info into submodule repo.

This copies the information about the submodules URL into the checked
out submodule config, acting like "git submodule sync". This is useful
if you have altered the URL for the submodule (or it has been altered
by a fetch of upstream changes) and you need to update your local repo.

## Signature

```ts
class Submodule {
  sync(signal?: AbortSignal | null | undefined): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Optional AbortSignal to cancel the operation.</p>
  </li>
</ul>