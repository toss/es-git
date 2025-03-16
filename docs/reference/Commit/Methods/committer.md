# committer

Get the committer of this commit.

## Signature

```ts
class Commit {
  committer(): Signature;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Signature</span>
    <br>
    <p class="param-description">Committer signature of this commit.</p>
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