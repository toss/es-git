# getMergeBasesMany

Find all merge bases given a list of commits

## Signature

```ts
class Repository {
  getMergeBasesMany(oids: string[]): string[];
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">oids</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">Oids of the commits.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string[]</span>
    <br>
    <p class="param-description">Array in which to store the resulting OIDs.</p>
  </li>
</ul>