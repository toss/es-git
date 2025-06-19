# getStatusFile

Get file status for a single file.

This tries to get status for the filename that you give. If no files
match that name (in either the HEAD, index, or working directory), this
returns NotFound.

If the name matches multiple files (for example, if the path names a
directory or if running on a case- insensitive filesystem and yet the
HEAD has two entries that both match the path), then this returns
Ambiguous because it cannot give correct results.

This does not do any sort of rename detection. Renames require a set of
targets and because of the path filtering, there is not enough
information to check renames correctly. To check file status with rename
detection, there is no choice but to do a full `statuses` and scan
through looking for the path that you are interested in.

## Signature

```ts
class Repository {
  getStatusFile(path: string): Status;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">A single file path.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Status</span>
    <br>
    <p class="param-description">The  <code>Status</code>  of this single file.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">conflicted</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">current</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">ignored</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexDeleted</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexModified</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexNew</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexRenamed</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">indexTypechange</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtDeleted</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtModified</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtNew</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtRenamed</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">wtTypechange</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">Throws error if the status file does not exist.</p>
  </li>
</ul>