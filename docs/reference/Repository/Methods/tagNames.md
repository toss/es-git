# tagNames

Get a list with all the tags in the repository.

## Signature

```ts
class Repository {
  tagNames(pattern?: string | null | undefined): string[];
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">pattern</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">An optional fnmatch pattern can also be specified.</p>
  </li>
</ul>