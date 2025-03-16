# getRemote

Get remote from repository.

## Signature

```ts
class Repository {
  getRemote(name: string): Remote;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Remote</span>
    <br>
    <p class="param-description">Remote instance.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if remote does not exist.</p>
  </li>
</ul>