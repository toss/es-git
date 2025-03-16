# tagger

Get the tagger (author) of a tag.

## Signature

```ts
class Tag {
  tagger(): Signature | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Signature</span>
    <br>
    <p class="param-description">If the author is unspecified, then  <code>null</code>  is returned.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Email on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">Name on the signature.</p>
      </li>
      <li class="param-li">
        <span class="param-name">timestamp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">Time in seconds, from epoch</p>
      </li>
    </ul>
  </li>
</ul>