# setSorting

Set the order in which commits are visited.

## Signature

```ts
class Revwalk {
  setSorting(sort: number): this;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">sort</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Orderings that may be specified for Revwalk iteration. - <code>RevwalkSort.None</code> : Sort the repository contents in no particular ordering. This sorting is arbitrary, implementation-specific, and subject to change at any time. This is the default sorting for new walkers. - <code>RevwalkSort.Topological</code> : Sort the repository contents in topological order (children before parents). This sorting mode can be combined with time sorting. - <code>RevwalkSort.Time</code> : Sort the repository contents by commit time. This sorting mode can be combined with topological sorting. - <code>RevwalkSort.Reverse</code> : Iterate through the repository contents in reverse order. This sorting mode can be combined with any others.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository, RevwalkSort } from 'es-git';

const repo = await openRepository('.');
const revwalk = repo.revwalk();
revwalk.setSorting(RevwalkSort.Time | RevwalkSort.Reverse);
```