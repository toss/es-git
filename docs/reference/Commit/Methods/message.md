# message

Get the full message of a commit.

The returned message will be slightly prettified by removing any
potential leading newlines.

Throws error if the message is not valid utf-8.

## Signature

```ts
class Commit {
  message(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Full message of this commit.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the message is not valid utf-8.</p>
  </li>
</ul>