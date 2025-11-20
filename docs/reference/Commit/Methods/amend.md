# amend

Amend this existing commit with all non-nullable values

This creates a new commit that is exactly the same as the old commit,
except that any non-nullable values will be updated. The new commit has
the same parents as the old commit.

## Signature

```ts
class Commit {
  amend(options?: AmendOptions, tree?: Tree): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | AmendOptions</span>
    <br>
    <p class="param-description">Options for amending commit.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">author</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">Signature for author.</p>
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
        <p class="param-description">Signature for committer.</p>
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
        <span class="param-name">message</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Full message for this commit</p>
      </li>
      <li class="param-li">
        <span class="param-name">messageEncoding</span><span class="param-type">string</span>
        <br>
        <p class="param-description">The encoding for the message in the commit, represented with a standard encoding name. E.g. &quot;UTF-8&quot;. If NULL, no encoding header is written and UTF-8 is assumed.</p>
      </li>
      <li class="param-li">
        <span class="param-name">updateRef</span><span class="param-type">string</span>
        <br>
        <p class="param-description">If not NULL, name of the reference that will be updated to point to this commit. If the reference is not direct, it will be resolved to a direct reference. Use &quot;HEAD&quot; to update the HEAD of the current branch and make it point to this commit.  If the reference doesn&#39;t exist yet, it will be created. If it does exist, the first parent must be the tip of this branch.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">tree</span><span class="param-type">null | Tree</span>
    <br>
    <p class="param-description">Tree to use for amending commit.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">string</span>
    <br>
    <p class="param-description">ID(SHA1) of amended commit.</p>
  </li>
</ul>