# getName

Lookup a tree entry by its filename.

## Signature

```ts
class Tree {
  getName(filename: string): TreeEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">filename</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Filename of tree entry.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">Tree entry.</p>
  </li>
</ul>