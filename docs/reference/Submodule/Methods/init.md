# init

Copy submodule info into ".git/config" file.

Just like "git submodule init", this copies information about the
submodule into ".git/config". You can use the accessor functions above
to alter the in-memory git_submodule object and control what is written
to the config, overriding what is in .gitmodules.

By default, existing entries will not be overwritten, but passing `true`
for `overwrite` forces them to be updated.

## Signature

```ts
class Submodule {
  init(
    overwrite?: boolean | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">overwrite</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">By default, existing entries will not be overwritten, but setting this to true forces them to be updated.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Optional AbortSignal to cancel the operation.</p>
  </li>
</ul>