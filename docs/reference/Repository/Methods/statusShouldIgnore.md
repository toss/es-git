# statusShouldIgnore

Test if the ignore rules apply to a given file.

This function checks the ignore rules to see if they would apply to the
given file. This indicates if the file would be ignored regardless of
whether the file is already in the index or committed to the repository.

One way to think of this is if you were to do "git add ." on the
directory containing the file, would it be added or not?

## Signature

```ts
class Repository {
  statusShouldIgnore(path: string): boolean;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">A given file path.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">Returns  <code>true</code>  if the ignore rules apply to a given file.</p>
  </li>
</ul>