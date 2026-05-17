# get

Gets a status entry from this list at the specified index.

## Signature

```ts
class Statuses {
  get(index: number): StatusEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">index</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Index of the status entry to get.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">StatusEntry | null</span>
    <br>
    <p class="param-description">A status entry from this list at the specified index. Returns  <code>null</code>  if the status<br>entry does not exist.</p>
  </li>
</ul>