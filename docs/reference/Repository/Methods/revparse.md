# revparse

Execute a rev-parse operation against the `spec` listed.

The resulting revision specification is returned, or an error is
returned if one occurs.

## Signature

```ts
class Repository {
  revparse(spec: string): Revspec;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">spec</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Revision string.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Revspec</span>
    <br>
    <p class="param-description"></p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">from</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Access the <code>from</code> range of this revspec.</p>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">Returns the intent of the revspec.</p>
      </li>
      <li class="param-li">
        <span class="param-name">to</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Access the <code>to</code> range of this revspec.</p>
      </li>
    </ul>
  </li>
</ul>