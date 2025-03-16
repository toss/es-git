# pushRange

Push and hide the respective endpoints of the given range.

## Signature

```ts
class Revwalk {
  pushRange(range: string): this;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">range</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The range should be of the form <code>&lt;commit&gt;..&lt;commit&gt;</code> where each <code>&lt;commit&gt;</code> is in the form accepted by <code>revparseSingle</code>. The left-hand commit will be hidden and the right-hand commit pushed.</p>
  </li>
</ul>