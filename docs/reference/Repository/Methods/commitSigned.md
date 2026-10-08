# commitSigned

Create a signed commit from externally signed commit content.

This writes the signed commit to the object database but does not update
`HEAD` or any other reference. Use `commit` with `signature` and `updateRef`
to also move a reference. If `signatureField` is omitted, Git's default
`gpgsig` field is used. The signature itself is not verified.
A single trailing newline is removed from the signature before storing it.

## Signature

```ts
class Repository {
  commitSigned(commitContent: string, signature: string, signatureField?: string | null | undefined): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">commitContent</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">Commit content returned by <code>commitCreateBuffer</code>.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signature</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">ASCII-armored signature over <code>commitContent</code>, such as the output of <code>gpg --detach-sign --armor</code> or <code>ssh-keygen -Y sign -n git</code>.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signatureField</span><span class="param-type">string | null</span>
    <br>
    <p class="param-description">Header field name. Defaults to <code>gpgsig</code>; must not be empty or contain whitespace or NUL bytes.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">ID(SHA1) of created commit.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If the commit content cannot be parsed, its tree or a parent does not exist,<br>an argument contains a NUL byte, or the signature field is empty or contains whitespace.</p>
  </li>
</ul>

## Examples

```ts
import { execFileSync } from 'node:child_process';

// Requires GPG with a signing key configured.
const content = repo.commitCreateBuffer(tree, 'signed commit', {
  parents: [repo.head().target()!],
});
const signature = execFileSync('gpg', ['--detach-sign', '--armor'], {
  input: Buffer.from(content, 'utf8'),
}).toString('utf8');
const oid = repo.commitSigned(content, signature); // HEAD is unchanged.
```