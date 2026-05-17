# push

Perform a push.

Perform all the steps for a push.
If no refspecs are passed, then the configured refspecs will be used.

## Signature

```ts
class Remote {
  push(
    refspecs: string[],
    options?: PushOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refspecs</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">Refspecs to push to remote.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">PushOptions | null</span>
    <br>
    <p class="param-description">Options for push remote.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">callbacks</span><span class="param-type">RemoteCallbacks</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">packProgress</span><span class="param-type">(stage: PackBuilderStage, current: number, total: number) =&gt; void</span>
            <br>
            <p class="param-description">Function to call with progress information during pack building.  Be aware that this is called inline with pack building operations, so performance may be affected.</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushNegotiation</span><span class="param-type">(update: PushUpdate[]) =&gt; void</span>
            <br>
            <p class="param-description">The callback is called once between the negotiation step and the upload.  The argument to the callback is a slice containing the updates which will be sent as commands to the destination.</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushTransferProgress</span><span class="param-type">(current: number, total: number, bytes: number) =&gt; void</span>
            <br>
            <p class="param-description">The callback through which progress of push transfer is monitored</p>
          </li>
          <li class="param-li">
            <span class="param-name">pushUpdateReference</span><span class="param-type">(refname: string, status: string | null) =&gt; void</span>
            <br>
            <p class="param-description">Set a callback to get invoked for each updated reference on a push.  The first argument to the callback is the name of the reference and the second is a status message sent by the server. If the status is not <code>null</code> then the push was rejected.</p>
          </li>
          <li class="param-li">
            <span class="param-name">sidebandProgress</span><span class="param-type">(data: Uint8Array) =&gt; void</span>
            <br>
            <p class="param-description">Textual progress from the remote.  Text sent over the progress side-band will be passed to this function (this is the &#39;counting objects&#39; output).</p>
          </li>
          <li class="param-li">
            <span class="param-name">transferProgress</span><span class="param-type">(data: RemoteTransferProgress) =&gt; void</span>
            <br>
            <p class="param-description">Called with transfer progress during fetch.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateTips</span><span class="param-type">(refname: string, oldId: string, newId: string) =&gt; void</span>
            <br>
            <p class="param-description">Each time a reference is updated locally, the callback will be called with information about it.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">credential</span><span class="param-type">Credential</span>
        <br>
        <p class="param-description">A interface to represent git credentials in libgit2.</p>
      </li>
      <li class="param-li">
        <span class="param-name">customHeaders</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">Set extra headers for this push operation.</p>
      </li>
      <li class="param-li">
        <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
        <br>
        <p class="param-description">Set remote redirection settings; whether redirects to another host are permitted.  By default, git will follow a redirect on the initial request (<code>/info/refs</code>), but not subsequent requests.</p>
        <p class="param-description">- <code>None</code> : Do not follow any off-site redirects at any stage of the fetch or push.<br>- <code>Initial</code> : Allow off-site redirects only upon the initial request. This is the default.<br>- <code>All</code> : Allow redirects at any stage in the fetch or push.</p>
      </li>
      <li class="param-li">
        <span class="param-name">pbParallelism</span><span class="param-type">number</span>
        <br>
        <p class="param-description">If the transport being used to push to the remote requires the creation of a pack file, this controls the number of worker threads used by the packbuilder when creating that pack file to be sent to the remote.  If set to 0, the packbuilder will auto-detect the number of threads to create, and the default value is 1.</p>
      </li>
      <li class="param-li">
        <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
        <br>
        <p class="param-description">Set the proxy options to use for the push operation.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">auto</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Try to auto-detect the proxy from the git configuration.  Note that this will override <code>url</code> specified before.</p>
          </li>
          <li class="param-li">
            <span class="param-name">url</span><span class="param-type">string</span>
            <br>
            <p class="param-description">Specify the exact URL of the proxy to use.  Note that this will override <code>auto</code> specified before.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">remoteOptions</span><span class="param-type">string[]</span>
        <br>
        <p class="param-description">Set &quot;push options&quot; to deliver to the remote.</p>
      </li>
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">AbortSignal | null</span>
    <br>
    <p class="param-description">Abort signal.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('/path/to/repo');
const remote = repo.getRemote('origin');

// Push the local "main" branch to the remote "other" branch
await remote.push(['refs/heads/main:refs/heads/other']);

// Push with credential.
await remote.push(['refs/heads/main:refs/heads/other'], {
  credential: {
    type: 'Plain',
    password: '<personal access token>',
  },
});
```