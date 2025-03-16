# Tags

## Listing Tags

To retrieve the names of all tags in a repository, you can use [`tagNames()`](../reference/Repository/Methods/tagNames.md).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const tagNames = repo.tagNames();
console.log(tagNames); // ["v0", "v1", "v2"]
```

To get both the tag names and the associated tag hashes, you can iterate through all tags using [`tagForeach()`](../reference/Repository/Methods/tagForeach.md).

```ts
const tags = [];
repo.tagForeach((sha, name) => {
  tags.push([name, sha]);
  return true;
});
console.log(tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
//  ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2']]

// If you wish to stop iteration, return `false` in the callback.
const v0to1Tags = [];
repo.tagForeach((sha, name) => {
  v0to1Tags.push([name, sha]);
  return name !== 'refs/tags/v1';
});
console.log(v0to1Tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1']]
```

And then, get the tag with [`getTag()`](../reference/Repository/Methods/getTag.md).

```ts
const tag = repo.getTag('aa00405');
console.log(tag.id()); // "aa0040546ed22b8bb33f3bd621e8d10ed849b02c"
console.log(tag.name()); // "v0"
console.log(tag.message()); // "message for this tag"

// Get the commit pointed to by the tag.
const commit = tag.target().peelToCommit();
console.log(commit.id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
```

## Creating Tags

You can create a tag for a specific commit by using [`createTag()`](../reference/Repository/Methods/createTag.md).

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const commit = repo.getCommit('828954df9f08dc8e172447cdacf0ddea1adf9e63');

const tagSha = repo.createTag('mytag', commit.asObject(), 'this is my tag', {
  tagger: {
    name: 'Seokju Na',
    email: 'seokju.me@toss.im'
  },
});
const tag = repo.getTag(tagSha);
console.log(tag.name()); // "mytag"
console.log(tag.target().id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
```

If a tag with the same name already exists, an error will occur. If you want to replace an existing tag with a new one, use the `force` option.

```ts
const tagSha = repo.createTag('mytag', commit.asObject(), 'this is my tag', {
  // The existing tag will be replaced by the newly created tag.
  force: true,
});
```
