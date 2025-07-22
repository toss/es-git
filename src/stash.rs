use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

#[napi(object)]
#[derive(Default)]
/// Options for saving a stash.
///
/// All fields are optional. If not provided, sensible defaults will be used.
///
/// @example
/// ```ts
/// import { openRepository } from 'es-git';
///
/// const repo = await openRepository('./path/to/repo');
///
/// // Basic usage
/// repo.stashSave({
///   stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' }
/// });
///
/// // With options
/// repo.stashSave({
///   stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
///   message: 'WIP: feature implementation',
///   includeUntracked: true
/// });
/// ```
pub struct StashSaveOptions {
  /// The identity of the person performing the stashing.
  /// If not provided, uses the repository's default signature.
  pub stasher: Option<SignaturePayload>,
  /// Description along with the stashed state.
  /// If not provided, a default message will be generated.
  pub message: Option<String>,
  /// Whether to stash untracked files.
  /// Default: false
  pub include_untracked: Option<bool>,
  /// Whether to stash ignored files.
  /// Default: false
  pub include_ignored: Option<bool>,
  /// Whether to retain the index after stashing.
  /// If true, staged changes remain in the index after stashing.
  /// Default: false
  pub keep_index: Option<bool>,
}

#[napi(object)]
/// Options for applying a stash.
///
/// Controls how a stash is applied to the working directory.
///
/// @example
/// ```ts
/// import { openRepository } from 'es-git';
///
/// const repo = await openRepository('./path/to/repo');
///
/// // Default apply
/// repo.stashApply(0);
///
/// // With options
/// repo.stashApply(0, { reinstantiateIndex: true });
/// ```
pub struct StashApplyOptions {
  /// Whether to reinstall the index from the stash.
  /// If true, the index state recorded in the stash is also restored.
  /// Default: false
  pub reinstantiate_index: Option<bool>,
}

impl From<StashApplyOptions> for git2::StashApplyOptions<'_> {
  fn from(value: StashApplyOptions) -> Self {
    let mut opts = git2::StashApplyOptions::new();
    if let Some(true) = value.reinstantiate_index {
      opts.reinstantiate_index();
    }
    opts
  }
}

pub(crate) enum StashEntryInner {
  StashList(SharedReference<StashList, (usize, git2::Oid, Option<String>)>),
  Owned((usize, git2::Oid, Option<String>)),
}

impl StashEntryInner {
  fn index(&self) -> usize {
    match self {
      Self::StashList(list) => list.0,
      Self::Owned((index, _, _)) => *index,
    }
  }

  fn oid(&self) -> &git2::Oid {
    match self {
      Self::StashList(list) => &list.1,
      Self::Owned((_, oid, _)) => oid,
    }
  }

  fn message(&self) -> Option<&str> {
    match self {
      Self::StashList(list) => list.2.as_deref(),
      Self::Owned((_, _, message)) => message.as_deref(),
    }
  }
}

#[napi]
/// A class to represent a git stash entry.
///
/// A stash entry represents a snapshot of the working directory and index that has been saved
/// temporarily. Each stash entry has an index (position in the stash stack), an ID (commit SHA),
/// and an optional message describing the changes.
pub struct StashEntry {
  pub(crate) inner: StashEntryInner,
}

#[napi]
impl StashEntry {
  #[napi]
  /// Get the index of this stash entry.
  ///
  /// The index represents the position of this stash in the stash stack, where 0 is the most recent stash.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashEntry {
  ///   index(): number;
  /// }
  /// ```
  ///
  /// @returns {number} Index of this stash entry (0-based, with 0 being the most recent).
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  /// const stash = stashList.get(0);
  /// console.log(stash?.index()); // 0
  /// ```
  pub fn index(&self) -> u32 {
    self.inner.index() as u32
  }

  #[napi]
  /// Get the id (SHA1) of this stash entry.
  ///
  /// Each stash is stored as a commit object, and this returns the commit SHA.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashEntry {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns {string} The 40-character hexadecimal SHA1 hash of the stash commit.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  /// const stash = stashList.get(0);
  /// console.log(stash?.id()); // e.g., "a1b2c3d4e5f6..."
  /// ```
  pub fn id(&self) -> String {
    self.inner.oid().to_string()
  }

  #[napi]
  /// Get the message of this stash entry.
  ///
  /// Returns the message associated with the stash when it was created. If no custom message
  /// was provided, it returns the default message generated by Git.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashEntry {
  ///   message(): string | null;
  /// }
  /// ```
  ///
  /// @returns {string | null} The stash message, or null if not available.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  /// const stash = stashList.get(0);
  /// console.log(stash?.message()); // e.g., "WIP on main: abc1234 fix: typo"
  /// ```
  pub fn message(&self) -> Option<String> {
    self.inner.message().map(|s| s.to_string())
  }
}

#[napi]
/// A container for a list of stash entries about a repository.
///
/// The stash list provides access to all stashes in the repository. Stashes are indexed
/// from 0 (most recent) to n-1 (oldest). This class provides methods to access individual
/// stashes, check the count, and iterate over all stashes.
///
/// @example
/// ```ts
/// import { openRepository } from 'es-git';
///
/// const repo = await openRepository('./path/to/repo');
/// const stashList = repo.stashList();
/// console.log(`Total stashes: ${stashList.len()}`);
///
/// // Iterate over all stashes
/// for (const stash of stashList.iter()) {
///   console.log(`${stash.index()}: ${stash.message()}`);
/// }
/// ```
pub struct StashList {
  pub(crate) inner: SharedReference<Repository, Vec<(usize, git2::Oid, Option<String>)>>,
}

#[napi]
impl StashList {
  #[napi]
  /// Gets a stash entry from this list at the specified index.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashList {
  ///   get(index: number): StashEntry | null;
  /// }
  /// ```
  ///
  /// @param {number} index - Index of the stash entry to get (0-based, where 0 is the most recent).
  /// @returns {StashEntry | null} A stash entry from this list at the specified index, or `null` if the index is out of bounds.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  ///
  /// // Get the most recent stash
  /// const stash = stashList.get(0);
  /// if (stash) {
  ///   console.log(stash.message());
  /// }
  /// ```
  pub fn get(&self, index: u32, this: Reference<StashList>, env: Env) -> Option<StashEntry> {
    let idx = index as usize;
    if idx >= self.inner.len() {
      return None;
    }

    this
      .share_with(env, move |list| {
        list
          .inner
          .get(idx)
          .cloned()
          .ok_or(Error::new(napi::Status::GenericFailure, "index out of bounds"))
      })
      .ok()
      .map(|entry| StashEntry {
        inner: StashEntryInner::StashList(entry),
      })
  }

  #[napi]
  /// Gets the count of stash entries in this list.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashList {
  ///   len(): number;
  /// }
  /// ```
  ///
  /// @returns If there are no stashes in the repository, this should return 0.
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Check if the stash list is empty.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashList {
  ///   isEmpty(): boolean;
  /// }
  /// ```
  ///
  /// @returns {boolean} Returns `true` if there are no stash entries in this repository.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  ///
  /// if (stashList.isEmpty()) {
  ///   console.log('No stashes found');
  /// } else {
  ///   console.log(`Found ${stashList.len()} stashes`);
  /// }
  /// ```
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Returns an iterator over the stash entries in this list.
  ///
  /// The iterator yields stash entries in order from newest (index 0) to oldest.
  ///
  /// @category Stash/Methods
  /// @signature
  /// ```ts
  /// class StashList {
  ///   iter(): StashListIter;
  /// }
  /// ```
  ///
  /// @returns {StashListIter} An iterator that yields StashEntry objects.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  ///
  /// // Iterate over stashes
  /// for (const stash of stashList.iter()) {
  ///   console.log(`${stash.index()}: ${stash.message()}`);
  /// }
  /// ```
  pub fn iter(&self, this: Reference<StashList>, env: Env) -> crate::Result<StashListIter> {
    let entries = self.inner.deref().clone();
    let inner = this.share_with(env, move |_| Ok(entries))?;
    Ok(StashListIter { inner, index: 0 })
  }
}

#[napi(iterator)]
pub struct StashListIter {
  pub(crate) inner: SharedReference<StashList, Vec<(usize, git2::Oid, Option<String>)>>,
  index: usize,
}

#[napi]
impl Generator for StashListIter {
  type Yield = StashEntry;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    let current_index = self.index;
    if current_index >= self.inner.len() {
      return None;
    }

    self.index += 1;

    self.inner.get(current_index).map(|entry| StashEntry {
      inner: StashEntryInner::Owned(entry.clone()),
    })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Save the local modifications to a new stash.
  ///
  /// This method saves your current working directory and index state to a new stash entry,
  /// allowing you to temporarily store changes and work on something else. The working directory
  /// is reverted to match the HEAD commit after stashing.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   stashSave(options?: StashSaveOptions): string;
  /// }
  /// ```
  ///
  /// @param {StashSaveOptions} [options] - Options for saving the stash.
  /// @returns {string} The object ID (40-character SHA1) of the commit containing the stashed state.
  /// @throws {Error} If there are no local changes to stash or if the stash operation fails.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  ///
  /// // Simple stash
  /// const stashId = repo.stashSave({
  ///   stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  ///   message: 'WIP: implementing new feature'
  /// });
  ///
  /// // Stash including untracked files
  /// repo.stashSave({
  ///   stasher: { name: 'Seokju Na', email: 'seokju.me@toss.im' },
  ///   includeUntracked: true
  /// });
  /// ```
  pub fn stash_save(&mut self, options: Option<StashSaveOptions>) -> crate::Result<String> {
    let options = options.unwrap_or_default();

    let stasher = options
      .stasher
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;

    let mut flags = git2::StashFlags::empty();
    if options.include_untracked.unwrap_or(false) {
      flags |= git2::StashFlags::INCLUDE_UNTRACKED;
    }
    if options.include_ignored.unwrap_or(false) {
      flags |= git2::StashFlags::INCLUDE_IGNORED;
    }
    if options.keep_index.unwrap_or(false) {
      flags |= git2::StashFlags::KEEP_INDEX;
    }

    let oid = self
      .inner
      .stash_save2(&stasher, options.message.as_deref(), Some(flags))?;
    Ok(oid.to_string())
  }

  #[napi]
  /// Apply a single stashed state from the stash list.
  ///
  /// This method applies the changes from a stash entry to your working directory.
  /// Unlike `stashPop`, this does not remove the stash from the list after applying.
  /// Conflicts may occur if the stashed changes conflict with the current working directory.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   stashApply(index: number, options?: StashApplyOptions): void;
  /// }
  /// ```
  ///
  /// @param {number} index - The index of the stash to apply (0 is the most recent).
  /// @param {StashApplyOptions} [options] - Options for applying the stash.
  /// @throws {Error} If the stash index is invalid or if there are conflicts during application.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  ///
  /// // Apply the most recent stash
  /// repo.stashApply(0);
  ///
  /// // Apply with options
  /// repo.stashApply(0, { reinstantiateIndex: true });
  /// ```
  pub fn stash_apply(&mut self, index: u32, options: Option<StashApplyOptions>) -> crate::Result<()> {
    let idx = index as usize;
    match options {
      Some(o) => {
        let mut opts = o.into();
        self.inner.stash_apply(idx, Some(&mut opts))?;
      }
      None => {
        self.inner.stash_apply(idx, None)?;
      }
    }
    Ok(())
  }

  #[napi]
  /// Remove a single stashed state from the stash list.
  ///
  /// This permanently deletes a stash entry. The stash is removed from the list and
  /// cannot be recovered. All subsequent stashes will be reindexed (e.g., stash@{2}
  /// becomes stash@{1} after dropping stash@{1}).
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   stashDrop(index: number): void;
  /// }
  /// ```
  ///
  /// @param {number} index - The index of the stash to drop (0 is the most recent).
  /// @throws {Error} If the stash index is invalid.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  ///
  /// // Drop the most recent stash
  /// repo.stashDrop(0);
  ///
  /// // Drop the third stash
  /// repo.stashDrop(2);
  /// ```
  pub fn stash_drop(&mut self, index: u32) -> crate::Result<()> {
    self.inner.stash_drop(index as usize)?;
    Ok(())
  }

  #[napi]
  /// Apply a single stashed state from the stash list and remove it from the list if successful.
  ///
  /// This method combines `stashApply` and `stashDrop` into a single operation. It applies
  /// the stash to your working directory and, if successful, removes it from the stash list.
  /// If the application fails (e.g., due to conflicts), the stash remains in the list.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   stashPop(index: number, options?: StashApplyOptions): void;
  /// }
  /// ```
  ///
  /// @param {number} index - The index of the stash to pop (0 is the most recent).
  /// @param {StashApplyOptions} [options] - Options for applying the stash.
  /// @throws {Error} If the stash index is invalid or if there are conflicts during application.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  ///
  /// // Pop the most recent stash
  /// repo.stashPop(0);
  ///
  /// // Pop with options
  /// repo.stashPop(0, { reinstantiateIndex: true });
  /// ```
  pub fn stash_pop(&mut self, index: u32, options: Option<StashApplyOptions>) -> crate::Result<()> {
    let idx = index as usize;
    match options {
      Some(o) => {
        let mut opts = o.into();
        self.inner.stash_pop(idx, Some(&mut opts))?;
      }
      None => {
        self.inner.stash_pop(idx, None)?;
      }
    }
    Ok(())
  }

  #[napi]
  /// Get the list of stash states in the repository.
  ///
  /// Returns a StashList object that provides access to all stashes in the repository.
  /// The list is ordered with the most recent stash at index 0.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   stashList(): StashList;
  /// }
  /// ```
  ///
  /// @returns {StashList} A container providing access to all stash entries in the repository.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const stashList = repo.stashList();
  ///
  /// if (!stashList.isEmpty()) {
  ///   console.log(`Found ${stashList.len()} stashes`);
  ///   for (const stash of stashList.iter()) {
  ///     console.log(`${stash.index()}: ${stash.message()}`);
  ///   }
  /// }
  /// ```
  pub fn stash_list(&self, this: Reference<Repository>, env: Env) -> crate::Result<StashList> {
    let inner = this.share_with(env, move |repo| {
      let mut entries = Vec::new();
      repo
        .inner
        .stash_foreach(|index, message, oid| {
          let message_str = Some(message.to_string());
          entries.push((index, *oid, message_str));
          true
        })
        .map_err(crate::Error::from)?;
      Ok(entries)
    })?;
    Ok(StashList { inner })
  }
}
