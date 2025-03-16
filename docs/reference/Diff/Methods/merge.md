# merge

Merge one diff into another.

This merges items from the "from" list into the "self" list.  The
resulting diff will have all items that appear in either list.
If an item appears in both lists, then it will be "merged" to appear
as if the old version was from the "onto" list and the new version
is from the "from" list (with the exception that if the item has a
pending DELETE in the middle, then it will show as deleted).

## Signature

```ts
class Diff {
  merge(diff: Diff): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">diff</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Diff</span>
    <br>
    <p class="param-description">Another diff to merge.</p>
  </li>
</ul>