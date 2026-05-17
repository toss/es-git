# iter

Gets all blame hunks as an iterator

## Signature

```ts
class Blame {
  iter(): Generator<BlameHunk>;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunks</span>
    <br>
    <p class="param-description">Iterator of all blame hunks</p>
  </li>
</ul>

## Examples

```ts
// Using for...of loop
for (const hunk of blame.iter()) {
  console.log(hunk.finalCommitId);
}

// Using spread operator to collect all hunks
const hunks = [...blame.iter()];
```