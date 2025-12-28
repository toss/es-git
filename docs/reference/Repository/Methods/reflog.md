# reflog

Lookup a reflog by its name.

## Signature

```ts
class Repository {
  reflog(name: string): Reflog;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Name of the reference whose reflog to lookup (e.g., &quot;HEAD&quot;, &quot;refs/heads/main&quot;).</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Reflog</span>
    <br>
    <p class="param-description">Reflog instance for the given reference name.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the reflog does not exist or cannot be opened.</p>
  </li>
</ul>