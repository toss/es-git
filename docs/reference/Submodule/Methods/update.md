# update

Update submodule.

This will clone a missing submodule and check out the subrepository to
the commit specified in the index of the containing repository. If
the submodule repository doesn't contain the target commit, then the
submodule is fetched using the fetch options supplied in options.

## Signature

```ts
class Submodule {
  update(
    init?: boolean | null | undefined,
    options?: SubmoduleUpdateOptions | null | undefined,
    signal?: AbortSignal | null | undefined,
  ): Promise<void>;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">init</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">Indicates if the submodule should be initialized first if it has not been initialized yet.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | SubmoduleUpdateOptions</span>
    <br>
    <p class="param-description">Configuration options for the update.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">allowFetch</span><span class="param-type">boolean</span>
        <br>
        <p class="param-description">Allow fetching from the submodule&#39;s default remote if the target commit isn&#39;t found. Default: <code>true</code>.</p>
      </li>
      <li class="param-li">
        <span class="param-name">checkout</span><span class="param-type">CheckoutOptions</span>
        <br>
        <p class="param-description">These options are passed to the checkout step.</p>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">allowConflicts</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">In safe mode, apply safe file updates even when there are conflicts instead of canceling the checkout.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ancestorLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">The name of the common ancestor side of conflicts</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleDiff3</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicates whether to include common ancestor data in diff3 format files for conflicts.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">conflictStyleMerge</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether a normal merge file should be written for conflicts.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dirPerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">Set the mode with which new directories are created.  Default is 0755</p>
          </li>
          <li class="param-li">
            <span class="param-name">disableFilters</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether to apply filters like CRLF conversion.</p>
          </li>
          <li class="param-li">
            <span class="param-name">disablePathspecMatch</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Treat paths specified in <code>path</code> as exact file paths instead of as pathspecs.</p>
          </li>
          <li class="param-li">
            <span class="param-name">dryRun</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate that this checkout should perform a dry run by checking for conflicts but not make any actual changes.</p>
          </li>
          <li class="param-li">
            <span class="param-name">filePerm</span><span class="param-type">number</span>
            <br>
            <p class="param-description">Set the mode with which new files are created.  The default is 0644 or 0755 as dictated by the blob.</p>
          </li>
          <li class="param-li">
            <span class="param-name">force</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Take any action necessary to get the working directory to match the target including potentially discarding modified files.</p>
          </li>
          <li class="param-li">
            <span class="param-name">ourLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">The name of the common our side of conflicts</p>
          </li>
          <li class="param-li">
            <span class="param-name">overwriteIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether ignored files should be overwritten during the checkout.  Defaults to true.</p>
          </li>
          <li class="param-li">
            <span class="param-name">path</span><span class="param-type">string</span>
            <br>
            <p class="param-description">Add a path to be checked out.  The path is a &lt;a href=&quot;https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec&quot;&gt;pathspec&lt;/a&gt; pattern, unless <code>disablePathspecMatch</code> is set.  If no paths are specified, then all files are checked out. Otherwise only these specified paths are checked out.</p>
          </li>
          <li class="param-li">
            <span class="param-name">recreateMissing</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">In safe mode, create files that don&#39;t exist.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">refresh</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether the index and git attributes should be refreshed from disk before any operations.  Defaults to true,</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeIgnored</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Remove ignored files from the working dir.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">removeUntracked</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Remove untracked files from the working dir.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">safe</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate that the checkout should be performed safely, allowing new files to be created but not overwriting existing files or changes.  This is the default.</p>
          </li>
          <li class="param-li">
            <span class="param-name">skipUnmerged</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Skip files with unmerged index entries.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">targetDir</span><span class="param-type">string</span>
            <br>
            <p class="param-description">Set the directory to check out to</p>
          </li>
          <li class="param-li">
            <span class="param-name">theirLabel</span><span class="param-type">string</span>
            <br>
            <p class="param-description">The name of the common their side of conflicts</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateIndex</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Prevents checkout from writing the updated files&#39; information to the index.  Defaults to true.</p>
          </li>
          <li class="param-li">
            <span class="param-name">updateOnly</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Only update the contents of files that already exist.  If set, files will not be created or deleted.  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useOurs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether the checkout should proceed on conflicts by using the stage 2 version of the file (&quot;ours&quot;).  Defaults to false.</p>
          </li>
          <li class="param-li">
            <span class="param-name">useTheirs</span><span class="param-type">boolean</span>
            <br>
            <p class="param-description">Indicate whether the checkout should proceed on conflicts by using the stage 3 version of the file (&quot;theirs&quot;).  Defaults to false.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">fetch</span><span class="param-type">FetchOptions</span>
        <br>
        <p class="param-description">Options which control the fetch, including callbacks.</p>
        <ul class="param-ul">
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
    </ul>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">signal</span><span class="param-type">null | AbortSignal</span>
    <br>
    <p class="param-description">Optional AbortSignal to cancel the operation.</p>
  </li>
</ul>