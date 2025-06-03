# addIgnoreRule

Add ignore rules for a repository.

This adds ignore rules to the repository. The rules will be used
in addition to any existing ignore rules (such as .gitignore files).

## Signature

```ts
class Repository {
  addIgnoreRule(rules: string): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">rules</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Rules to add, separated by newlines.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
repo.addIgnoreRule("node_modules/");
```