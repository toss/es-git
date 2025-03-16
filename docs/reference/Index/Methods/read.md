# read

Update the contents of an existing index object in memory by reading
from the hard disk.

## Signature

```ts
class Index {
  read(force?: boolean): void;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">force</span><span class="param-type">null | boolean</span>
    <br>
    <p class="param-description">If force is <code>true</code>, this performs a &quot;hard&quot; read that discards in-memory changes and always reloads the on-disk index data. If there is no on-disk version, the index will be cleared.  If force is <code>false</code>, this does a &quot;soft&quot; read that reloads the index data from disk only if it has changed since the last time it was loaded. Purely in-memory index data will be untouched. Be aware: if there are changes on disk, unwritten in-memory changes are discarded.</p>
  </li>
</ul>