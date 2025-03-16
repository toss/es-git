# getId

Lookup a tree entry by SHA value.

## Signature

```ts
class Tree {
  getId(id: string): TreeEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">SHA value.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | TreeEntry</span>
    <br>
    <p class="param-description">Tree entry with the given ID(SHA1).</p>
  </li>
</ul>