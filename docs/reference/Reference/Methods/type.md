# type

Get the reference type of a reference.

## Signature

```ts
class Reference {
  type(): ReferenceType | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | ReferenceType</span>
    <br>
    <p class="param-description">Returns  <code>null</code>  if the type is unknown.</p>
    <p class="param-description">- <code>Direct</code> : A reference which points at an object id.<br>- <code>Symbolic</code> : A reference which points at another reference.</p>
  </li>
</ul>