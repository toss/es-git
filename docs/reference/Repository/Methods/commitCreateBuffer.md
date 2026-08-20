# commitCreateBuffer

Create commit content for external signing.

This creates the unsigned commit object content without writing it to the
object database. Sign the exact UTF-8 content returned by this method,
then pass both values to `commitSigned`. Changing whitespace or line
endings after signing invalidates the signature.

## Signature

```ts
class Repository {
  commitCreateBuffer(
    tree: Tree,
    message: string,
    options?: CommitCreateBufferOptions | null | undefined,
  ): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
    <p class="param-description">Tree object to create commit content from.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Commit message.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">CommitCreateBufferOptions | null</span>
    <br>
    <p class="param-description">Options for creating commit content.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">Signature for author.  If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
      <li class="param-li">
        <span class="param-name">committer</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">Signature for committer.  If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
      <li class="param-li">
        <span class="param-name">parents</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">Parent commit IDs in their intended order.</p>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">Commit content to sign externally.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If a signature cannot be resolved or a parent commit does not exist.</p>
  </li>
</ul>

## Examples

```ts
const content = repo.commitCreateBuffer(tree, 'signed commit', {
  parents: [repo.head().target()!],
});
const signature = await signingBackend.sign(Buffer.from(content, 'utf8'));
const oid = repo.commitSigned(content, signature);
```