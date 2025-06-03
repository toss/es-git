# isPathIgnored

Test if the ignore rules apply to a given path.

This function checks the ignore rules to see if they would apply to the given file.
This indicates if the file would be ignored regardless of whether the file is already in the index or committed to the repository.

## Signature

```ts
class Repository {
  isPathIgnored(path: string): boolean;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The path to check.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">boolean</span>
    <br>
    <p class="param-description">- True if the path is ignored, false otherwise.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
const isIgnored = repo.isPathIgnored("node_modules/some-package");
console.log(`Path is ${isIgnored ? "ignored" : "not ignored"}`);
```