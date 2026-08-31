# getHunkByLine

Gets blame information for the specified line

## Signature

```ts
class Blame {
  getHunkByLine(line: number): BlameHunk;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">line</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
    <br>
    <p class="param-description">Line number to get blame information for (1-based)</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">BlameHunk</span>
    <br>
    <p class="param-description">Blame information for the specified line</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">finalCommitId</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">The oid of the commit where this line was last changed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">finalSignature</span><span class="param-type">Signature</span>
        <br>
        <p class="param-description">The signature of the commit where this line was last changed.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">Email on the signature.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">Name on the signature.</p>
          </li>
          <li class="param-li">
            <span class="param-name">offset</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">Timezone offset, in minutes</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">Time in seconds, from epoch</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">finalStartLineNumber</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">The 1-based line number in the final file where this hunk starts.</p>
      </li>
      <li class="param-li">
        <span class="param-name">isBoundary</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
        <p class="param-description">True if the hunk has been determined to be a boundary commit (the commit when the file was first introduced to the repository).</p>
      </li>
      <li class="param-li">
        <span class="param-name">linesInHunk</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">The number of lines in this hunk.</p>
      </li>
      <li class="param-li">
        <span class="param-name">origCommitId</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
        <p class="param-description">The oid of the commit where this line was originally written.</p>
      </li>
      <li class="param-li">
        <span class="param-name">origSignature</span><span class="param-type">Signature</span>
        <br>
        <p class="param-description">The signature of the commit where this line was originally written.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">email</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">Email on the signature.</p>
          </li>
          <li class="param-li">
            <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
            <br>
            <p class="param-description">Name on the signature.</p>
          </li>
          <li class="param-li">
            <span class="param-name">offset</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">Timezone offset, in minutes</p>
          </li>
          <li class="param-li">
            <span class="param-name">timestamp</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
            <br>
            <p class="param-description">Time in seconds, from epoch</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">origStartLineNumber</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
        <p class="param-description">The 1-based line number in the original file where this hunk starts.</p>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The path to the file where this line was originally written.</p>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If no hunk is found for the line</p>
  </li>
</ul>