# message

Get the message of this reflog entry.

## Signature

```ts
class ReflogEntry {
  message(): string;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string</span>
    <br>
    <p class="param-description">Message of this reflog entry. Returns  <code>null</code>  if no message is present.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the message is not valid utf-8.</p>
  </li>
</ul>