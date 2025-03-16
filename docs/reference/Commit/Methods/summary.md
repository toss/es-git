# summary

Get the short "summary" of the git commit message.

The returned message is the summary of the commit, comprising the first
paragraph of the message with whitespace trimmed and squashed.

Throws error if the summary is not valid utf-8.

## Signature

```ts
class Commit {
  summary(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Short summary of this commit message.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the summary is not valid utf-8.</p>
  </li>
</ul>