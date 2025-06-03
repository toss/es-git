# getMergeBases

Find all merge bases between two commits

## Signature

```ts
class Repository {
  getMergeBases(one: string, two: string): string[];
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">one</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">One of the commits OID.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">two</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The other commit OID.</p>
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