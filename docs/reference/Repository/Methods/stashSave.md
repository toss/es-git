# stashSave

Save the local modifications to a new stash.

This method saves your current working directory and index state to a new stash entry,
allowing you to temporarily store changes and work on something else. The working directory
is reverted to match the HEAD commit after stashing.

## Signature

```ts
class Repository {
  stashSave(options?: StashSaveOptions): string;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">StashSaveOptions | null</span>
    <br>
    <p class="param-description">Options for saving the stash.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">includeIgnored</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Whether to stash ignored files. Default: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">includeUntracked</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Whether to stash untracked files. Default: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">keepIndex</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Whether to retain the index after stashing. If true, staged changes remain in the index after stashing. Default: false</p>
      </li>
      <li class="param-li">
        <span class="param-name">message</span><span class="param-type">string</span>
        <br>
        <p class="param-description">Description along with the stashed state. If not provided, a default message will be generated.</p>
      </li>
      <li class="param-li">
        <span class="param-name">stasher</span><span class="param-type">SignaturePayload</span>
        <br>
        <p class="param-description">The identity of the person performing the stashing. If not provided, uses the repository&#39;s default signature.</p>
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
    <p class="param-description">The object ID (40-character SHA1) of the commit containing the stashed state.</p>
  </li>
</ul>

### Errors

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Error</span>
    <br>
    <p class="param-description">If there are no local changes to stash or if the stash operation fails.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('./path/to/repo');

// Simple stash
const stashId = repo.stashSave({
  stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  message: 'WIP: implementing new feature'
});

// Stash including untracked files
repo.stashSave({
  stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  includeUntracked: true
});
```