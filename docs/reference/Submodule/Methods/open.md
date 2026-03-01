# open

Open the repository for a submodule.

This will only work if the submodule is checked out into the working
directory.

## Signature

```ts
class Submodule {
  open(signal?: AbortSignal | null | undefined): Promise<Repository>;
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

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Promise&lt;Repository&gt;</span>
    <br>
    <p class="param-description">The repository.</p>
  </li>
</ul>