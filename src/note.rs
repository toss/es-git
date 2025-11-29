use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use git2::Oid;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// A structure representing a [note][1] in git.
///
/// [1]: http://alblue.bandlem.com/2011/11/git-tip-of-week-git-notes.html
pub struct Note {
  pub(crate) inner: SharedReference<Repository, git2::Note<'static>>,
}

#[napi]
impl Note {
  #[napi]
  /// Get the note object's id
  ///
  /// @category Note/Methods
  /// @signature
  /// ```ts
  /// class Note {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns The note object's id.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the note author
  ///
  /// @category Note/Methods
  /// @signature
  /// ```ts
  /// class Note {
  ///   author(): Signature;
  /// }
  /// ```
  ///
  /// @returns The note author signature.
  pub fn author(&self) -> crate::Result<Signature> {
    let sig = self.inner.author();
    Signature::try_from(sig)
  }

  #[napi]
  /// Get the note committer
  ///
  /// @category Note/Methods
  /// @signature
  /// ```ts
  /// class Note {
  ///   committer(): Signature;
  /// }
  /// ```
  ///
  /// @returns The note committer signature.
  pub fn committer(&self) -> crate::Result<Signature> {
    let sig = self.inner.committer();
    Signature::try_from(sig)
  }

  #[napi]
  /// Get the note message as a string.
  ///
  /// @category Note/Methods
  /// @signature
  /// ```ts
  /// class Note {
  ///   message(): string;
  /// }
  /// ```
  ///
  /// @returns The note message as a string
  /// @throws Throws error if message is not utf-8.
  pub fn message(&self) -> crate::Result<String> {
    let message = std::str::from_utf8(self.inner.message_bytes())?;
    Ok(message.to_string())
  }
}

#[napi(object)]
#[derive(Default)]
pub struct FindNoteOptions {
  pub notes_ref: Option<String>,
}

#[napi(object)]
#[derive(Default)]
pub struct CreateNoteOptions {
  /// Signature of the notes commit author.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub author: Option<SignaturePayload>,
  /// Signature of the notes commit commiter.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub committer: Option<SignaturePayload>,
  /// canonical name of the reference to use.
  ///
  /// Defaults to "refs/notes/commits".
  pub notes_ref: Option<String>,
  /// Overwrite existing note.
  pub force: Option<bool>,
}

#[napi(object)]
#[derive(Default)]
pub struct DeleteNoteOptions {
  /// Signature of the notes commit author.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub author: Option<SignaturePayload>,
  /// Signature of the notes commit commiter.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub committer: Option<SignaturePayload>,
  /// canonical name of the reference to use.
  ///
  /// Defaults to "refs/notes/commits".
  pub notes_ref: Option<String>,
}

#[napi(iterator)]
/// An iterator over all of the notes within a repository.
pub struct Notes {
  pub(crate) inner: SharedReference<Repository, git2::Notes<'static>>,
}

#[napi(object)]
pub struct NoteIterItem {
  pub note_id: String,
  pub annotated_id: String,
}

#[napi]
impl Generator for Notes {
  type Yield = NoteIterItem;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().and_then(|x| {
      x.ok().map(|(note_id, annotated_id)| NoteIterItem {
        note_id: note_id.to_string(),
        annotated_id: annotated_id.to_string(),
      })
    })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Add a note for an object
  ///
  /// The `notesRef` argument is the canonical name of the reference to use,
  /// defaulting to "refs/notes/commits". If `force` is specified then
  /// previous notes are overwritten.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   note(oid: string, note: string, options?: CreateNoteOptions | null | undefined): string;
  /// }
  /// ```
  ///
  /// @param {string} oid - OID of the git object to decorate.
  /// @param {string} note - Content of the note to add for object oid.
  /// @param {CreateNoteOptions} [options] - Options for creating note.
  /// @returns OID for the note.
  pub fn note(&self, oid: String, note: String, options: Option<CreateNoteOptions>) -> crate::Result<String> {
    let opts = options.unwrap_or_default();
    let oid = Oid::from_str(&oid)?;
    let author = opts
      .author
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let committer = opts
      .committer
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let notes_ref = opts.notes_ref;
    let force = opts.force.unwrap_or_default();
    let note_oid = self
      .inner
      .note(&author, &committer, notes_ref.as_deref(), oid, &note, force)?;
    Ok(note_oid.to_string())
  }

  #[napi]
  /// Get the default notes reference for this repository
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   noteDefaultRef(): string;
  /// }
  /// ```
  ///
  /// @returns The default notes reference.
  pub fn note_default_ref(&self) -> crate::Result<String> {
    let default_ref = self.inner.note_default_ref()?;
    Ok(default_ref)
  }

  #[napi]
  /// Creates a new iterator for notes in this repository.
  ///
  /// The `notesRef` argument is the canonical name of the reference to use,
  /// defaulting to "refs/notes/commits".
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   notes(notesRef?: string | null | undefined): Notes;
  /// }
  /// ```
  ///
  /// @param {string} [notesRef] - The canonical name of the reference to use.
  /// @returns Iterator of all notes. The iterator returned yields pairs of `[string, string]`
  /// where first element is the id of the note and the second id is the id the note is annotating.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// for (const { noteId, annotatedId } of repo.notes()) {
  ///   const note = repo.getNote(noteId);
  ///   const commit = repo.getCommit(annotatedId);
  /// }
  /// ```
  pub fn notes(&self, this: Reference<Repository>, env: Env, notes_ref: Option<String>) -> crate::Result<Notes> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .notes(notes_ref.as_deref())
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Notes { inner })
  }

  #[napi]
  /// Read the note for an object.
  ///
  /// The `notesRef` argument is the canonical name of the reference to use,
  /// defaulting to "refs/notes/commits".
  ///
  /// The id specified is the Oid of the git object to read the note from.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getNote(id: string, options?: FindNoteOptions | null | undefined): Note;
  /// }
  /// ```
  ///
  /// @param {string} id - OID of the git object to read the note from.
  /// @param {FindNoteOptions} [options] - Options for finding note.
  /// @returns Instance of the note.
  /// @throws Throws error if note does not exists.
  pub fn get_note(
    &self,
    this: Reference<Repository>,
    env: Env,
    id: String,
    options: Option<FindNoteOptions>,
  ) -> crate::Result<Note> {
    let opts = options.unwrap_or_default();
    let oid = Oid::from_str(&id)?;
    let notes_ref = opts.notes_ref;
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_note(notes_ref.as_deref(), oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Note { inner })
  }

  #[napi]
  /// Read the note for an object.
  ///
  /// The `notesRef` argument is the canonical name of the reference to use,
  /// defaulting to "refs/notes/commits".
  ///
  /// The id specified is the Oid of the git object to read the note from.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findNote(id: string, options?: FindNoteOptions | null | undefined): Note | null;
  /// }
  /// ```
  ///
  /// @param {string} id - OID of the git object to read the note from.
  /// @param {FindNoteOptions} [options] - Options for finding note.
  /// @returns Instance of the note. If does not exists, returns `null`.
  pub fn find_note(
    &self,
    this: Reference<Repository>,
    env: Env,
    id: String,
    options: Option<FindNoteOptions>,
  ) -> Option<Note> {
    self.get_note(this, env, id, options).ok()
  }

  #[napi]
  /// Remove the note for an object.
  ///
  /// The `notesRef` argument is the canonical name of the reference to use,
  /// defaulting to "refs/notes/commits".
  ///
  /// The id specified is the Oid of the git object to remove the note from.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   deleteNote(id: string, options?: DeleteNoteOptions | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {string} id - OID of the git object to remove the note from.
  /// @param {DeleteNoteOptions} [options] - Options for deleting note.
  pub fn delete_note(&self, id: String, options: Option<DeleteNoteOptions>) -> crate::Result<()> {
    let opts = options.unwrap_or_default();
    let id = Oid::from_str(&id)?;
    let author = opts
      .author
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let committer = opts
      .committer
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let notes_ref = opts.notes_ref;
    self.inner.note_delete(id, notes_ref.as_deref(), &author, &committer)?;
    Ok(())
  }
}
