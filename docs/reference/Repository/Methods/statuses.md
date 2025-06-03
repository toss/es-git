# statuses

Gather file status information and populate the returned structure.

Note that if a pathspec is given in the options to filter the
status, then the results from rename detection (if you enable it) may
not be accurate. To do rename detection properly, this must be called
with no pathspec so that all files can be considered.

## Signature

```ts
class Repository {
  statuses(): Statuses;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Statuses</span>
    <br>
    <p class="param-description">A container for a list of status information about a repository.</p>
  </li>
</ul>