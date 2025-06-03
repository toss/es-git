# analyzeMergeForRef

Analyzes the given branch(es) and determines the opportunities for
merging them into a reference.

## Signature

```ts
class Repository {
  analyzeMergeForRef(ourRef: Reference, theirHeads: AnnotatedCommit[]): MergeAnalysisResult;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">ourRef</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">Reference</span>
    <br>
    <p class="param-description">The reference to perform the analysis from.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">theirHeads</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">AnnotatedCommit[]</span>
    <br>
    <p class="param-description">The heads to merge into.</p>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">MergeAnalysisResult</span>
    <br>
    <p class="param-description">Merge analysis result.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">analysis</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">MergeAnalysis</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">fastForward</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">The given merge input is a fast-forward from HEAD and no merge needs to be performed.  Instead, the client can check out the given merge input.</p>
          </li>
          <li class="param-li">
            <span class="param-name">none</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">No merge is possible.</p>
          </li>
          <li class="param-li">
            <span class="param-name">normal</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">A &quot;normal&quot; merge; both HEAD and the given merge input have diverged from their common ancestor. The divergent commits must be merged.</p>
          </li>
          <li class="param-li">
            <span class="param-name">unborn</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">The HEAD of the current repository is &quot;unborn&quot; and does not point to a valid commit.  No merge can be performed, but the caller may wish to simply set HEAD to the target commit(s).</p>
          </li>
          <li class="param-li">
            <span class="param-name">upToDate</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">All given merge inputs are reachable from HEAD, meaning the repository is up-to-date and no merge needs to be performed.</p>
          </li>
        </ul>
      </li>
      <li class="param-li">
        <span class="param-name">preference</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">MergePreference</span>
        <br>
        <ul class="param-ul">
          <li class="param-li">
            <span class="param-name">fastForwardOnly</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">There is a <code>merge.ff=only</code> configuration setting, suggesting that the user only wants fast-forward merges.</p>
          </li>
          <li class="param-li">
            <span class="param-name">noFastForward</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">There is a <code>merge.ff=false</code> configuration setting, suggesting that the user does not want to allow a fast-forward merge.</p>
          </li>
          <li class="param-li">
            <span class="param-name">none</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">boolean</span>
            <br>
            <p class="param-description">No configuration was found that suggests a preferred behavior for merge.</p>
          </li>
        </ul>
      </li>
    </ul>
  </li>
</ul>