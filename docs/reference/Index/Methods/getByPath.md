# getByPath

Get one of the entries in the index by its path.

## Signature

```ts
class Index {
  getByPath(path: string, stage?: IndexStage | null | undefined): IndexEntry | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Path to lookup entry.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">stage</span><span class="param-type">null | IndexStage</span>
    <br>
    <p class="param-description">Git index stage states.</p>
    <p class="param-description">- <code>Any</code> : Match any index stage.<br>- <code>Normal</code> : A normal staged file in the index.<br>- <code>Ancestor</code> : The ancestor side of a conflict.<br>- <code>Ours</code> : The &quot;ours&quot; side of a conflict.<br>- <code>Theirs</code> : The &quot;theirs&quot; side of a conflict.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | IndexEntry</span>
    <br>
    <p class="param-description">Index entry for the path.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">ctime</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Date</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">dev</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">fileSize</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">flags</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">flagsExtended</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">gid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">id</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">ino</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">mode</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">mtime</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Date</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Buffer</span>
        <br>
        <p class="param-description">The path of this index entry as a byte vector. Regardless of the current platform, the directory separator is an ASCII forward slash (<code>0x2F</code>). There are no terminating or internal NUL characters, and no trailing slashes. Most of the time, paths will be valid utf-8 — but not always. For more information on the path storage format, see [these git docs](https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124). Note that libgit2 will take care of handling the prefix compression mentioned there.</p>
      </li>
      <li class="param-li">
        <span class="param-name">uid</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">number</span>
        <br>
      </li>
    </ul>
  </li>
</ul>
