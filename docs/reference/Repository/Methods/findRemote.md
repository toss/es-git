# findRemote

Find remote from repository.

## Signature

```ts
class Repository {
  findRemote(name: string): Remote | null;
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
    <span class="param-type">null | Remote</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if remote does not exist.</p>
  </li>
</ul>