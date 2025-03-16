# status

Returns the status of this entry.

## Signature

```ts
class DiffDelta {
  status(): DeltaType;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">DeltaType</span>
    <br>
    <p class="param-description">The status of this entry.</p>
    <p class="param-description">- <code>Unmodified</code> : No changes.<br>- <code>Added</code> : Entry does not exist in an old version.<br>- <code>Deleted</code> : Entry does not exist in a new version.<br>- <code>Modified</code> : Entry content changed between old and new.<br>- <code>Renamed</code> : Entry was renamed between old and new.<br>- <code>Copied</code> : Entry was copied from another old entry.<br>- <code>Ignored</code> : Entry is ignored item in workdir.<br>- <code>Untracked</code> : Entry is untracked item in workdir.<br>- <code>Typechange</code> : Type of entry changed between old and new.<br>- <code>Unreadable</code> : Entry is unreadable.<br>- <code>Conflicted</code> : Entry in the index is conflicted.</p>
  </li>
</ul>