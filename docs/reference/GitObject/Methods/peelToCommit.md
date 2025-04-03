# peelToCommit

Recursively peel an object until a commit is found.

## Signature

```ts
class GitObject {
  peelToCommit(): Commit;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Commit</span>
    <br>
    <p class="param-description">Git commit.</p>
  </li>
</ul>