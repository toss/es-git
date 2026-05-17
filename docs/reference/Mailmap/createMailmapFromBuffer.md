# createMailmapFromBuffer

Create a mailmap from the contents of a string.

The format of the string should follow the rules of the mailmap file:
```
# Comment line (ignored)
Seokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>
```

## Signature

```ts
function createMailmapFromBuffer(content: string): Mailmap;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">content</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Content of the mailmap file</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Mailmap</span>
    <br>
    <p class="param-description">A new mailmap object</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">An error if operation failed</p>
  </li>
</ul>