# commitCreateBuffer

Create the raw content of a commit object for external signing.

This creates unsigned commit content without writing it to the object database.
Sign the exact UTF-8 bytes of the returned string, then use `commit` with
`signature` and `updateRef` to write the signed commit and move a reference.
Use `commitSigned` to write only the object without updating any reference.
Changing the content, including whitespace or line endings, invalidates the signature.

When using `commit`, pass the same tree, message, parents, author and committer
with fixed `timeOptions` to both calls. Otherwise timestamps can differ when
`commit` rebuilds the content. Neither method verifies the signature.

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
        <p class="param-description">Author identity (name, email and time).  If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
        <p class="param-description">Committer identity (name, email and time).  If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
        <p class="param-description">Parent commit IDs. The first parent is the commit this one follows; omit for a root commit.</p>
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
    <p class="param-description">If an author or committer identity is invalid, no default signature is<br>configured, or a parent commit does not exist.</p>
  </li>
</ul>

## Examples

```ts
import { execFileSync } from 'node:child_process';

// Requires GPG with a signing key configured.
const identity = {
  name: 'Seokju Na',
  email: 'seokju.me@toss.im',
  timeOptions: { timestamp: Math.floor(Date.now() / 1000), offset: 0 },
};
const options = { author: identity, committer: identity, parents: [repo.head().target()!] };
const content = repo.commitCreateBuffer(tree, 'signed commit', options);
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');
const oid = repo.commit(tree, 'signed commit', { ...options, signature, updateRef: 'HEAD' });
```