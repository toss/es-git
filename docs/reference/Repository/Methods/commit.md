# commit

Create new commit in the repository.

If the `updateRef` is not `null`, name of the reference that will be
updated to point to this commit. If the reference is not direct, it will
be resolved to a direct reference. Use "HEAD" to update the HEAD of the
current branch and make it point to this commit. If the reference
doesn't exist yet, it will be created. If it does exist, the first
parent must be the tip of this branch.

## Signature

```ts
class Repository {
  commit(tree: Tree, message: string, options?: CommitOptions | null | undefined): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Tree</span>
    <br>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">message</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | CommitOptions</span>
    <br>
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
        <p class="param-description">Signature for commiter.  If not provided, the default signature of the repository will be used. If there is no default signature set for the repository, an error will occur.</p>
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
      </li>
      <li class="param-li">
        <span class="param-name">updateRef</span><span class="param-type">string</span>
        <br>
      </li>
      <li class="param-li">
        <span class="param-name">signature</span><span class="param-type">string</span>
        <br>
        <p class="param-description">GPG signature string for signed commits. If provided, this will create a signed commit.</p>
      </li>
      <li class="param-li">
        <span class="param-name">signatureField</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Custom signature field name. If not provided, the default signature field (gpgsig) will be used.</p>
      </li>
    </ul>
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
