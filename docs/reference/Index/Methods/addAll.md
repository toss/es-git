# addAll

Add or update index entries matching files in the working directory.

## Signature

```ts
class Index {
  addAll(pathspecs: string[], options?: IndexAddAllOptions | null | undefined): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">pathspecs</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">A List of file names of shell glob patterns that will matched against files in the repository&#39;s working directory. Each file that matches will be added to the index (either updating an existing entry or adding a new entry).</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | IndexAddAllOptions</span>
    <br>
    <p class="param-description">Options for add or update index entries.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkPathspec</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">To emulate <code>git add -A</code> and generate an error if the pathspec contains the exact path of an ignored file (when not using <code>force</code>), add the <code>checkPathspec</code> flag. This checks that each entry in <code>pathspecs</code> that is an exact match to a filename on disk is either not ignored or already in the index. If this check fails, the function will return an error.</p>
      </li>
      <li class="param-li">
        <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">The <code>pathspecs</code> are a list of file names or shell glob patterns that will matched against files in the repository&#39;s working directory. Each file that matches will be added to the index (either updating an existing entry or adding a new entry). You can disable glob expansion and force exact matching with the <code>disablePathspecMatch</code> flag.</p>
      </li>
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Files that are ignored will be skipped (unlike <code>addPath</code>). If a file is already tracked in the index, then it will be updated even if it is ignored. Pass the <code>force</code> flag to skip the checking of ignore rules.</p>
      </li>
      <li class="param-li">
        <span class="param-name">onMatch</span><span class="param-type">(args: IndexOnMatchCallbackArgs) =&gt; number</span>
        <br>
        <p class="param-description">If you provide a callback function, it will be invoked on each matching item in the working directory immediately before it is added to / updated in the index. Returning zero will add the item to the index, greater than zero will skip the item, and less than zero will abort the scan an return an error to the caller.</p>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">This method will fail in bare index instances.</p>
  </li>
</ul>

## Examples

Emulate `git add *`:

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const index = repo.index();
index.addAll(['*']);
index.write();
```
