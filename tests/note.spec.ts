import { describe, expect, it } from 'vitest';
import { openRepository } from '../index';
import { useFixture } from './fixtures';

describe('note', () => {
  const signature = {
    name: 'Seokju Na',
    email: 'seokju.me@toss.im',
  };

  it('get default note ref', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const ref = repo.noteDefaultRef();
    expect(ref).toEqual('refs/notes/commits');
  });

  it('create note', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    const noteId = repo.note('a01e9888e46729ef4aa68953ba19b02a7a64eb82', 'this is note', {
      author: signature,
      committer: signature,
    });
    const note = repo.getNote('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
    expect(note.id()).toEqual(noteId);
    expect(note.message()).toEqual('this is note');
  });

  it('delete note', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    repo.note('a01e9888e46729ef4aa68953ba19b02a7a64eb82', 'this is note', {
      author: signature,
      committer: signature,
    });
    expect(repo.findNote('a01e9888e46729ef4aa68953ba19b02a7a64eb82')).not.toBeNull();
    repo.deleteNote('a01e9888e46729ef4aa68953ba19b02a7a64eb82', {
      author: signature,
      committer: signature,
    });
    expect(repo.findNote('a01e9888e46729ef4aa68953ba19b02a7a64eb82')).toBeNull();
  });

  it('iterate notes', async () => {
    const p = await useFixture('commits');
    const repo = await openRepository(p);
    repo.note('a01e9888e46729ef4aa68953ba19b02a7a64eb82', 'note1', {
      author: signature,
      committer: signature,
    });
    repo.note('b33e0101b828225f77eeff4dfa31259dcf379002', 'note2', {
      author: signature,
      committer: signature,
    });
    const result = [...repo.notes()];
    expect(result).toHaveLength(2);
  });
});
