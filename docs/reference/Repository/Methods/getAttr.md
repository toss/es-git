# getAttr

Get the value of a git attribute for a path.

## Signature

```ts
class Repository {
  getAttr(
    path: string,
    name: string,
    options?: AttrOptions | null | undefined
  ): boolean | string | Buffer | null;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">path</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The path to check for attributes. Relative paths are interpreted relative to the repo root.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of the attribute to look up.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | AttrOptions</span>
    <br>
    <p class="param-description">Options for attribute lookup.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">checkFileThenIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Check the working directory, then the index.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkIndexOnly</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Check the index only.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkIndexThenFile</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Check the index, then the working directory.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkNoSystem</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Do not use the system gitattributes file.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | string | boolean | Buffer&lt;ArrayBufferLike&gt;</span>
    <br>
    <p class="param-description">Output of the value of the attribute.</p>
  </li>
</ul>