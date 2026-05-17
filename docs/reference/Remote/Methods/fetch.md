# fetch

Download new data and update tips.

Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.

## Signature

```ts
class Remote {
  fetch(
    refspecs: string[],
    options?: FetchRemoteOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">refspecs</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string[]</span>
    <br>
    <p class="param-description">Refspecs to fetch from remote.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">FetchRemoteOptions | null</span>
    <br>
    <p class="param-description">Options for fetch remote.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">fetch</span><span class="param-type">FetchOptions</span>
        <br>
        <p class="param-description">Options which can be specified to various fetch operations.</p>
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
            <p class="param-description">Set extra headers for this fetch operation.</p>
          </li>
          <li class="param-li">
            <span class="param-name">depth</span><span class="param-type">number</span>
            <br>
            <p class="param-description">Set fetch depth, a value less or equal to 0 is interpreted as pull everything (effectively the same as not declaring a limit depth).</p>
          </li>
          <li class="param-li">
            <span class="param-name">downloadTags</span><span class="param-type">AutotagOption</span>
            <br>
            <p class="param-description">Set how to behave regarding tags on the remote, such as auto-downloading tags for objects we&#39;re downloading or downloading all of them.  The default is to auto-follow tags.</p>
            <p class="param-description">- <code>Unspecified</code> : Use the setting from the remote&#39;s configuration<br>- <code>Auto</code> : Ask the server for tags pointing to objects we&#39;re already downloading<br>- <code>None</code> : Don&#39;t ask for any tags beyond the refspecs<br>- <code>All</code> : Ask for all the tags</p>
          </li>
          <li class="param-li">
            <span class="param-name">followRedirects</span><span class="param-type">RemoteRedirect</span>
            <br>
            <p class="param-description">Set remote redirection settings; whether redirects to another host are permitted.  By default, git will follow a redirect on the initial request (<code>/info/refs</code>), but not subsequent requests.</p>
            <p class="param-description">- <code>None</code> : Do not follow any off-site redirects at any stage of the fetch or push.<br>- <code>Initial</code> : Allow off-site redirects only upon the initial request. This is the default.<br>- <code>All</code> : Allow redirects at any stage in the fetch or push.</p>
          </li>
          <li class="param-li">
            <span class="param-name">proxy</span><span class="param-type">ProxyOptions</span>
            <br>
            <p class="param-description">Set the proxy options to use for the fetch operation.</p>
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
            <span class="param-name">prune</span><span class="param-type">FetchPrune</span>
            <br>
            <p class="param-description">Set whether to perform a prune after the fetch.</p>
            <p class="param-description">- <code>Unspecified</code> : Use the setting from the configuration.<br>- <code>On</code> : Force pruning on.<br>- <code>Off</code> : Force pruning off</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">reflogMsg</span><span class="param-type">string</span>
        <br>
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

// Fetching data from the "main" branch
await remote.fetch(['main']);

// Providing an empty array fetches data using the default Refspec configured for the remote
await remote.fetch([]);
```