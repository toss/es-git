# findNote

Read the note for an object.

The `notesRef` argument is the canonical name of the reference to use,
defaulting to "refs/notes/commits".

The id specified is the Oid of the git object to read the note from.

## Signature

```ts
class Repository {
  findNote(id: string, options?: FindNoteOptions | null | undefined): Note | null;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">id</span><span class="param-required">required</span>&nbsp;·&nbsp;<span class="param-type">string</span>
    <br>
    <p class="param-description">OID of the git object to read the note from.</p>
  </li>
  <li class="param-li param-li-root">
    <span class="param-name">options</span><span class="param-type">null | FindNoteOptions</span>
    <br>
    <p class="param-description">Options for finding note.</p>
    <ul class="param-ul">
      <li class="param-li">
        <span class="param-name">notesRef</span><span class="param-type">string</span>
        <br>
      </li>
    </ul>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">null | Note</span>
    <br>
    <p class="param-description">Instance of the note. If does not exists, returns  <code>null</code> .</p>
  </li>
</ul>