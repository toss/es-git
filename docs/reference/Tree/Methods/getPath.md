# getPath

Retrieve a tree entry contained in a tree or in any of its subtrees,
given its relative path.

## Signature

```ts
class Tree {
  getPath(path: string): TreeEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Relative path to tree entry.</p>
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