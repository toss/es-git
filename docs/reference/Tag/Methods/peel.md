# peel

Recursively peel a tag until a non tag Git object is found.

## Signature

```ts
class Tag {
  peel(): GitObject;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object for this tag.</p>
  </li>
</ul>