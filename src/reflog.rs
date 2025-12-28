use std::ops::{Deref, DerefMut};

use git2::Oid;
use napi::bindgen_prelude::*;
use napi::{Env, Error};
use napi_derive::napi;

use crate::{repository::Repository, signature::Signature};

/// Internal implementation of ReflogEntry holding the original git2 value
pub(crate) enum ReflogEntryInner {
  Reflog(SharedReference<Reflog, git2::ReflogEntry<'static>>),
  Owned(git2::ReflogEntry<'static>),
}

impl Deref for ReflogEntryInner {
  type Target = git2::ReflogEntry<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Reflog(inner) => inner.deref(),
      Self::Owned(inner) => inner,
    }
  }
}

#[napi]
/// A class to represent a git reflog entry.
pub struct ReflogEntry {
  pub(crate) inner: ReflogEntryInner,
}

#[napi]
impl ReflogEntry {
  #[napi]
  /// Get the committer of this reflog entry.
  ///
  /// @category ReflogEntry/Methods
  ///
  /// @signature
  /// ```ts
  /// class ReflogEntry {
  ///   committer(): Signature;
  /// }
  /// ```
  ///
  /// @returns Committer signature of this reflog entry.
  pub fn committer(&self) -> crate::Result<Signature> {
    Signature::try_from(self.inner.committer())
  }

  #[napi]
  /// Get the new object ID (SHA1) of this reflog entry.
  ///
  /// @category ReflogEntry/Methods
  ///
  /// @signature
  /// ```ts
  /// class ReflogEntry {
  ///   idNew(): string;
  /// }
  /// ```
  ///
  /// @returns New object ID (SHA1) of this reflog entry.
  pub fn id_new(&self) -> String {
    self.inner.id_new().to_string()
  }

  #[napi]
  /// Get the old object ID (SHA1) of this reflog entry.
  ///
  /// @category ReflogEntry/Methods
  ///
  /// @signature
  /// ```ts
  /// class ReflogEntry {
  ///   idOld(): string;
  /// }
  /// ```
  ///
  /// @returns Old object ID (SHA1) of this reflog entry.
  pub fn id_old(&self) -> String {
    self.inner.id_old().to_string()
  }

  #[napi]
  /// Get the message of this reflog entry.
  ///
  /// @category ReflogEntry/Methods
  ///
  /// @signature
  /// ```ts
  /// class ReflogEntry {
  ///   message(): string | null;
  /// }
  /// ```
  ///
  /// @returns Message of this reflog entry. Returns `null` if no message is present.
  /// @throws Throws error if the message is not valid utf-8.
  pub fn message(&self) -> crate::Result<Option<String>> {
    match self.inner.message_bytes() {
      None => Ok(None),
      Some(bytes) => {
        let message = std::str::from_utf8(bytes)?;
        Ok(Some(message.to_string()))
      }
    }
  }
}

#[napi(iterator)]
/// An iterator over the entries in a reflog.
pub struct ReflogIter {
  pub(crate) inner: SharedReference<Reflog, git2::ReflogIter<'static>>,
}

#[napi]
impl Generator for ReflogIter {
  type Yield = ReflogEntry;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().map(|entry| ReflogEntry {
      inner: ReflogEntryInner::Owned(entry),
    })
  }
}

/// Internal implementation of Reflog holding the original git2 value
pub(crate) enum ReflogInner {
  Owned(git2::Reflog),
}

impl Deref for ReflogInner {
  type Target = git2::Reflog;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Owned(inner) => inner,
    }
  }
}

impl DerefMut for ReflogInner {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Self::Owned(inner) => inner,
    }
  }
}

#[napi]
/// A class to represent a git reflog.
pub struct Reflog {
  pub(crate) inner: ReflogInner,
}

#[napi]
impl Reflog {
  #[napi]
  /// Append a new entry to the reflog.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   append(newOid: string, committer: Signature, msg?: string | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {string} newOid - New object ID (SHA1) for this reflog entry.
  /// @param {Signature} committer - Committer signature for this reflog entry.
  /// @param {string} [msg] - Optional message for this reflog entry.
  /// @throws Throws error if the OID is invalid or if appending fails.
  pub fn append(&mut self, new_oid: String, committer: Signature, msg: Option<String>) -> crate::Result<()> {
    let git2_new_oid = Oid::from_str(&new_oid)?;
    let git2_committer: git2::Signature<'_> = git2::Signature::try_from(committer)?;
    self.inner.append(git2_new_oid, &git2_committer, msg.as_deref())?;
    Ok(())
  }

  #[napi]
  /// Remove an entry from the reflog.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   remove(i: number, rewritePreviousEntry?: boolean): void;
  /// }
  /// ```
  ///
  /// @param {number} i - Index of the entry to remove.
  /// @param {boolean} [rewritePreviousEntry] - Whether to rewrite the previous entry. Defaults to `false`.
  /// @throws Throws error if the index is invalid or if removal fails.
  pub fn remove(&mut self, i: u32, rewrite_previous_entry: Option<bool>) -> crate::Result<()> {
    self.inner.remove(i as usize, rewrite_previous_entry.unwrap_or(false))?;
    Ok(())
  }

  #[napi]
  /// Get a reflog entry by index.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   get(i: number): ReflogEntry | null;
  /// }
  /// ```
  ///
  /// @param {number} i - Index of the entry to get.
  /// @returns Reflog entry at the given index. Returns `null` if the index is out of bounds.
  pub fn get(&self, this: Reference<Reflog>, env: Env, i: u32) -> Option<ReflogEntry> {
    if i as usize >= self.inner.len() {
      return None;
    }

    this
      .share_with(env, |reflog| {
        reflog
          .inner
          .get(i as usize)
          .ok_or(Error::new(Status::InvalidArg, "invalid index"))
      })
      .ok()
      .map(|inner| ReflogEntry {
        inner: ReflogEntryInner::Reflog(inner),
      })
  }

  #[napi]
  /// Get the number of entries in the reflog.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   len(): number;
  /// }
  /// ```
  ///
  /// @returns Number of entries in the reflog.
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Check if the reflog is empty.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   isEmpty(): boolean;
  /// }
  /// ```
  ///
  /// @returns `true` if the reflog is empty, `false` otherwise.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Create an iterator over the entries in the reflog.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   iter(): ReflogIter;
  /// }
  /// ```
  ///
  /// @returns Iterator over the reflog entries.
  /// @throws Throws error if the reflog cannot be accessed.
  pub fn iter(&self, this: Reference<Reflog>, env: Env) -> crate::Result<ReflogIter> {
    let inner = this.share_with(env, |reflog| Ok(reflog.inner.iter()))?;
    Ok(ReflogIter { inner })
  }

  #[napi]
  /// Write the reflog to disk.
  ///
  /// @category Reflog/Methods
  ///
  /// @signature
  /// ```ts
  /// class Reflog {
  ///   write(): void;
  /// }
  /// ```
  ///
  /// @throws Throws error if writing fails.
  pub fn write(&mut self) -> crate::Result<()> {
    self.inner.write().map_err(crate::Error::from)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reflog by its name.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   reflog(name: string): Reflog;
  /// }
  /// ```
  ///
  /// @param {string} name - Name of the reference whose reflog to lookup (e.g., "HEAD", "refs/heads/main").
  /// @returns Reflog instance for the given reference name.
  /// @throws Throws error if the reflog does not exist or cannot be opened.
  pub fn reflog(&self, name: String) -> crate::Result<Reflog> {
    let inner = self.inner.reflog(&name).map_err(crate::Error::from)?;
    Ok(Reflog {
      inner: ReflogInner::Owned(inner),
    })
  }

  #[napi]
  /// Rename a reflog.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   reflogRename(oldName: string, newName: string): void;
  /// }
  /// ```
  ///
  /// @param {string} oldName - Old name of the reference.
  /// @param {string} newName - New name of the reference.
  /// @throws Throws error if renaming fails.
  pub fn reflog_rename(&self, old_name: String, new_name: String) -> crate::Result<()> {
    self
      .inner
      .reflog_rename(&old_name, &new_name)
      .map_err(crate::Error::from)
  }

  #[napi]
  /// Delete a reflog.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   reflogDelete(name: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - Name of the reference whose reflog to delete.
  /// @throws Throws error if deletion fails.
  pub fn reflog_delete(&self, name: String) -> crate::Result<()> {
    self.inner.reflog_delete(&name).map_err(crate::Error::from)
  }
}
