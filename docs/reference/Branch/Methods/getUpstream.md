# getUpstream

Return the reference supporting the remote tracking branch, given a
local branch reference.

## Signature

```ts
class Branch {
  getUpstream(): Branch;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Branch</span>
    <br>
    <p class="param-description">The reference supporting the remote tacking branch.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if upstream does not exist.</p>
  </li>
</ul>