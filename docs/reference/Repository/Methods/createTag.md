# createTag

Create a new tag in the repository from an object.

A new reference will also be created pointing to this tag object.

The message will not be cleaned up.

The tag name will be checked for validity. You must avoid the characters
'~', '^', ':', ' \ ', '?', '[', and '*', and the sequences ".." and " @
{" which have special meaning to revparse.

## Signature

```ts
class Repository {
  createTag(
    name: string,
    target: GitObject,
    message: string,
    options?: CreateTagOptions | null | undefined,
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
    <span class="param-name">message</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">The message of tag.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CreateTagOptions</span>
    <br>
    <p class="param-description">Options for creating the tag.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">force</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">If <code>force</code> is true and a reference already exists with the given name, it&#39;ll be replaced.</p>
      </li>
      <li class="param-li">
        <span class="param-name">tagger</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">Signature for tagger.  If not provided, default signature of repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
            <span class="param-name">timeOptions</span><span class="param-type">SignatureTimeOptions</span>
            <br>
            <ul class="param-ul">
              <li class="param-li">
                <span class="param-name">offset</span><span class="param-type">number</span>
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
        </ul>
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

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
const commit = repo.getCommit('828954df9f08dc8e172447cdacf0ddea1adf9e63');

const sha = repo.createTag(
  'mytag',
  commit.asObject(),
  'this is my tag message',
  {
    tagger: {
      name: 'Seokju Na',
      email: 'seokju.me@toss.im',
    },
  },
);
const tag = repo.getTag(sha);
console.log(tag.name()); // "mytag"
console.log(tag.target().id()); // "828954df9f08dc8e172447cdacf0ddea1adf9e63"
```
