# getMergeBaseOctopus

Find a merge base in preparation for an octopus merge.

## Signature

```ts
class Repository {
  getMergeBaseOctopus(oids: string[]): string;
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
    <span class="param-type">string</span>
    <br>
    <p class="param-description">The OID of a merge base considering all the commits.</p>
  </li>
</ul>