# notes

Creates a new iterator for notes in this repository.

The `notesRef` argument is the canonical name of the reference to use,
defaulting to "refs/notes/commits".

## Signature

```ts
class Repository {
  notes(noteRef?: string | null | undefined): Notes;
}
```

### Parameters

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">notesRef</span><span class="param-type">null | string</span>
    <br>
  </li>
</ul>

### Returns

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Notes</span>
    <br>
    <p class="param-description">Iterator of all notes. The iterator returned yields pairs of  <code>[string, string]</code> <br>where first element is the id of the note and the second id is the id the note is annotating.</p>
  </li>
</ul>

## Examples

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
for (const value of repo.notes()) {
  const [noteId, annotatedId] = value;
  const note = repo.getNote(noteId);
  const commit = repo.getCommit(annotatedId);
}
```