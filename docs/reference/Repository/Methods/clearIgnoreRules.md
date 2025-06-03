# clearIgnoreRules

Clear ignore rules that were explicitly added.

Resets to the default internal ignore rules.
This will not turn off rules in .gitignore files that actually exist in the filesystem.
The default internal ignores ignore ".", ".." and ".git" entries.

## Signature

```ts
class Repository {
  clearIgnoreRules(): void;
}
```

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');
repo.addIgnoreRule("*.log");
// Later, clear all added rules
repo.clearIgnoreRules();
```