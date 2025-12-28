# get

Get a reflog entry by index.

## Signature

```ts
class Reflog {
  get(i: number): ReflogEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">i</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Index of the entry to get.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ReflogEntry</span>
    <br>
    <p class="param-description">Reflog entry at the given index. Returns  <code>null</code>  if the index is out of bounds.</p>
  </li>
</ul>