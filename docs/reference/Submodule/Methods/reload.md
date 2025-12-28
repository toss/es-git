# reload

Reread submodule info from config, index, and `HEAD`.

Call this to reread cached submodule information for this submodule if
you have reason to believe that it has changed.

## Signature

```ts
class Submodule {
  reload(
    force?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">force</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">If this is <code>true</code>, then data will be reloaded even if it doesn&#39;t seem out of date.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Optional AbortSignal to cancel the operation.</p>
  </li>
</ul>