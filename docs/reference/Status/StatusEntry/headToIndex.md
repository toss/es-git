# headToIndex

Access detailed information about the differences between the file in
`HEAD` and the file in the index.

## Signature

```ts
class StatusEntry {
  headToIndex(): DiffDelta | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DiffDelta | null</span>
    <br>
    <p class="param-description">The differences between the file in  <code>HEAD</code>  and the file in the index.</p>
  </li>
</ul>