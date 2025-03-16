# hideGlob

Hide matching references.

The OIDs pointed to by the references that match the given glob pattern
and their ancestors will be hidden from the output on the revision walk.

## Signature

```ts
class Revwalk {
  hideGlob(glob: string): this;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">glob</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">A leading &#39;refs/&#39; is implied if not present as well as a trailing <code>/ \ *</code> if the glob lacks &#39;?&#39;, &#39; \ *&#39; or &#39;[&#39;. Any references matching this glob which do not point to a commitish will be ignored.</p>
  </li>
</ul>