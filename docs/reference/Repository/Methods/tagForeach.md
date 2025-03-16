# tagForeach

Iterate over all tags calling `callback` on each.
The callback is provided the tag id and name.

## Signature

```ts
class Repository {
  tagForeach(callback: (oid: string, name: string) => boolean): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">(oid: string, name: string) =&gt; boolean</span>
    <br>
    <p class="param-description">If you wish to stop iteration, return <code>false</code> in the callback.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const tags = [];
repo.tagForeach((sha, name) => {
  tags.push([name, sha]);
  return true;
});

console.log(tags);
// [['aa0040546ed22b8bb33f3bd621e8d10ed849b02c', 'refs/tags/v0'],
//  ['674e3327707fcf32a348ecfc0cb6b93e57398b8c', 'refs/tags/v1'],
//  ['567aa5c6b219312dc7758ab88ebb7a1e5d36d26b', 'refs/tags/v2']]
```