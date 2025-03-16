# deleteTag

Delete an existing tag reference.

## Signature

```ts
class Repository {
  deleteTag(name: string): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The tag name will be checked for validity, see <code>isValidTagName</code> for some rules about valid names.</p>
  </li>
</ul>