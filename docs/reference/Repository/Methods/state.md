# state

Returns the current state of this repository.

## Signature

```ts
class Repository {
  state(): RepositoryState;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">RepositoryState</span>
    <br>
    <p class="param-description">The current state of this repository.</p>
    <p class="param-description">Available states are <code>Clean</code>, <code>Merge</code>, <code>Revert</code>, <code>RevertSequence</code>, <code>CherryPick</code>,<br><code>CherryPickSequence</code>, <code>Bisect</code>, <code>Rebase</code>, <code>RebaseInteractive</code>, <code>RebaseMerge</code>,<br><code>ApplyMailbox</code>, <code>ApplyMailboxOrRebase</code>.</p>
  </li>
</ul>