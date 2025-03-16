# index

Get the Index file for this repository.

If a custom index has not been set, the default index for the repository
will be returned (the one located in `.git/index`).

## Signature

```ts
class Repository {
  index(): Index;
}
```

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Index</span>
    <br>
    <p class="param-description">The index file for this repository.</p>
  </li>
</ul>