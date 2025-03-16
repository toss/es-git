# body

Get the long "body" of the git commit message.

The returned message is the body of the commit, comprising everything
but the first paragraph of the message. Leading and trailing whitespaces
are trimmed.

Throws error if the summary is not valid utf-8.

## Signature

```ts
class Commit {
  body(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Long body of this commit message.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the body is not valid utf-8.</p>
  </li>
</ul>