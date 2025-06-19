# blameFile

Creates a blame object for the file at the given path

## Signature

```ts
class Repository {
  blameFile(path: string, options?: BlameOptions): Blame;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Path to the file to blame</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | BlameOptions</span>
    <br>
    <p class="param-description">Options to control blame behavior</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">firstParent</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Restrict search to commits reachable following only first parents.</p>
      </li>
      <li class="param-li">
        <span class="param-name">ignoreWhitespace</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Ignore whitespace differences.</p>
      </li>
      <li class="param-li">
        <span class="param-name">maxLine</span><span class="param-type">number</span>
        <br>
        <p class="param-description">The maximum line number to blame (1-based index)</p>
      </li>
      <li class="param-li">
        <span class="param-name">minLine</span><span class="param-type">number</span>
        <br>
        <p class="param-description">The minimum line number to blame (1-based index)</p>
      </li>
      <li class="param-li">
        <span class="param-name">newestCommit</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The oid of the newest commit to consider. The blame algorithm will stop when this commit is reached.</p>
      </li>
      <li class="param-li">
        <span class="param-name">oldestCommit</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The oid of the oldest commit to consider. The blame algorithm will stop when this commit is reached.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The path to the file being worked on. Path has to be relative to the repo root.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesAnyCommitCopies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Track lines that have been copied from another file that exists in any commit.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesSameCommitCopies</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Track lines that have been copied from another file that exists in the same commit.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackCopiesSameCommitMoves</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Track lines that have moved across files in the same commit.</p>
      </li>
      <li class="param-li">
        <span class="param-name">trackLinesMovement</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Track lines that have moved within a file. This is the git-blame -M option.</p>
      </li>
      <li class="param-li">
        <span class="param-name">useMailmap</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Use mailmap file to map author and committer names and email addresses to canonical real names and email addresses.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Blame</span>
    <br>
    <p class="param-description">Blame object for the specified file</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the file doesn&#39;t exist or can&#39;t be opened</p>
  </li>
</ul>

## Examples

```ts
// Blame the entire file
const blame = repo.blameFile('path/to/file.js');

// Blame a single line
const lineBlame = repo.blameFile('path/to/file.js', { minLine: 10, maxLine: 10 });

// Blame a range of lines
const rangeBlame = repo.blameFile('path/to/file.js', { minLine: 5, maxLine: 15 });
```