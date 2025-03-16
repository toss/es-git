# isValidTagName

Determine whether a tag name is valid, meaning that (when prefixed with refs/tags/) that
it is a valid reference name, and that any additional tag name restrictions are imposed
(eg, it cannot start with a -).

## Signature

```ts
function isValidTagName(tagName: string): boolean;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tagName</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Tag name to check if it is valid.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  if tag name is valid.</p>
  </li>
</ul>