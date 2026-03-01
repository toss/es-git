# notes

이 리포지토리에서 노트용 새 이터레이터를 만들어요.

`notesRef` 인수는 사용할 참조의 표준 이름이고,
기본값은 "refs/notes/commits"예요.

## 시그니처

```ts
class Repository {
  notes(notesRef?: string | null | undefined): Notes;
}
```

### 파라미터

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-name">notesRef</span><span class="param-type">null | string</span>
    <br>
    <p class="param-description">사용할 참조의 표준 이름이에요.</p>
  </li>
</ul>

### 반환 값

<ul class="param-ul">
  <li class="param-li param-li-root">
    <span class="param-type">Notes</span>
    <br>
    <p class="param-description">모든 노트에 대한 이터레이터예요. 반환된 이터레이터는 <code>[string, string]</code> <br>쌍을 순서대로 제공하고, 첫 번째 요소는 노트의 ID이고 두 번째 요소는 그 노트가 주석을 다는 대상의 ID예요.</p>
  </li>
</ul>

## 예제

```ts
import { openRepository } from 'es-git';

const repo = await openRepository('.');
for (const { noteId, annotatedId } of repo.notes()) {
  const note = repo.getNote(noteId);
  const commit = repo.getCommit(annotatedId);
}
```