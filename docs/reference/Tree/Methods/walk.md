# walk

Traverse the entries in a tree and its subtrees in post or pre-order.

## Signature

```ts
class Tree {
  walk(mode: TreeWalkMode, callback: (entry: TreeEntry) => number): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">mode</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">TreeWalkMode</span>
    <br>
    <p class="param-description">A indicator of whether a tree walk should be performed in pre-order or post-order.</p>
    <p class="param-description">- <code>PreOrder</code> : Runs the traversal in pre-order.<br>- <code>PostOrder</code> : Runs the traversal in post-order.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">(entry: TreeEntry) =&gt; number</span>
    <br>
    <p class="param-description">The callback function will be run on each node of the tree that&#39;s walked. The return code of this function will determine how the walk continues. <code>libgit2</code> requires that the callback be an integer, where 0 indicates a successful visit, 1 skips the node, and -1 aborts the traversal completely.</p>
  </li>
</ul>