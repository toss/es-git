# traceSet

Set the global subscriber called when libgit2 produces a tracing message.

## Signature

```ts
function traceSet(
  level: TraceLevel,
  callback: (level: TraceLevel, message: string) => void,
): void;
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">level</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">TraceLevel</span>
    <br>
    <p class="param-description">Level to set tracing to</p>
    <p class="param-description">Available tracing levels. When tracing is set to a particular level,<br>callers will be provided tracing at the given level and all lower levels.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">callback</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">(level: TraceLevel, message: string) =&gt; void</span>
    <br>
    <p class="param-description">Callback to call with trace data</p>
  </li>
</ul>