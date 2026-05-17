# iterByLine

Collects blame hunks by scanning file lines as an iterator

## Signature

```ts
class Blame {
  iterByLine(): Generator<BlameHunk>;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunksByLine</span>
    <br>
    <p class="param-description">Iterator of blame hunks collected by line scanning</p>
  </li>
</ul>

## Examples

```ts
// Using for...of loop
for (const hunk of blame.iterByLine()) {
  console.log(hunk.finalCommitId);
}

// Using spread operator to collect all hunks
const hunks = [...blame.iterByLine()];
```