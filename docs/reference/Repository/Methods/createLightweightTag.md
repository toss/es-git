# createLightweightTag

Create a new lightweight tag pointing at a target object.

A new direct reference will be created pointing to this target object.

## Signature

```ts
class Repository {
  createLightweightTag(
    name: string,
    target: GitObject,
    options?: CreateLightweightTagOptions,
  ): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">name</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The name of tag.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">target</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">GitObject</span>
    <br>
    <p class="param-description">Git object to pointed by this tag.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateLightweightTagOptions</span>
    <br>
    <p class="param-description">Options for creating the tag.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If <code>force</code> is true and a reference already exists with the given name, it&#39;ll be replaced.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Tag OID(SHA1) which created.</p>
  </li>
</ul>