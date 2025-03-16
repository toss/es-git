# toObject

Convert a tree entry to the object it points to.

## Signature

```ts
class TreeEntry {
  toObject(repo: Repository): GitObject;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">repo</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Repository</span>
    <br>
    <p class="param-description">Repository which this tree entry belongs to.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object that pointed by the entry.</p>
  </li>
</ul>