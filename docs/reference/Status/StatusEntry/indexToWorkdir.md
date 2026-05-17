# indexToWorkdir

Access detailed information about the differences between the file in
the index and the file in the working directory.

## Signature

```ts
class StatusEntry {
  indexToWorkdir(): DiffDelta | null;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DiffDelta | null</span>
    <br>
    <p class="param-description">Differences between the file in the index and the file in the working directory.</p>
  </li>
</ul>