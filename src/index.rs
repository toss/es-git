use crate::repository::Repository;
use crate::util;
use chrono::{DateTime, Timelike, Utc};
use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
pub struct IndexEntry {
  pub ctime: DateTime<Utc>,
  pub mtime: DateTime<Utc>,
  pub dev: u32,
  pub ino: u32,
  pub mode: u32,
  pub uid: u32,
  pub gid: u32,
  pub file_size: u32,
  pub id: String,
  pub flags: u16,
  pub flags_extended: u16,
  /// The path of this index entry as a byte vector. Regardless of the
  /// current platform, the directory separator is an ASCII forward slash
  /// (`0x2F`). There are no terminating or internal NUL characters, and no
  /// trailing slashes. Most of the time, paths will be valid utf-8 — but
  /// not always. For more information on the path storage format, see
  /// [these git docs][git-index-docs]. Note that libgit2 will take care of
  /// handling the prefix compression mentioned there.
  ///
  /// [git-index-docs]: https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124
  pub path: Buffer,
}

impl TryFrom<git2::IndexEntry> for IndexEntry {
  type Error = crate::Error;

  fn try_from(value: git2::IndexEntry) -> std::result::Result<Self, Self::Error> {
    let ctime = DateTime::<Utc>::from_timestamp(value.ctime.seconds() as i64, value.ctime.nanoseconds())
      .ok_or(crate::Error::InvalidTime)?;
    let mtime = DateTime::<Utc>::from_timestamp(value.mtime.seconds() as i64, value.mtime.nanoseconds())
      .ok_or(crate::Error::InvalidTime)?;
    Ok(Self {
      ctime,
      mtime,
      dev: value.dev,
      ino: value.ino,
      mode: value.mode,
      uid: value.uid,
      gid: value.gid,
      file_size: value.file_size,
      id: value.id.to_string(),
      flags: value.flags,
      flags_extended: value.flags_extended,
      path: Buffer::from(value.path),
    })
  }
}

impl TryFrom<IndexEntry> for git2::IndexEntry {
  type Error = crate::Error;

  fn try_from(value: IndexEntry) -> std::result::Result<Self, Self::Error> {
    Ok(git2::IndexEntry {
      ctime: git2::IndexTime::new(value.ctime.second() as i32, value.ctime.nanosecond()),
      mtime: git2::IndexTime::new(value.mtime.second() as i32, value.mtime.nanosecond()),
      dev: value.dev,
      ino: value.ino,
      mode: value.mode,
      uid: value.uid,
      gid: value.gid,
      file_size: value.file_size,
      id: git2::Oid::from_str(&value.id)?,
      flags: value.flags,
      flags_extended: value.flags_extended,
      path: value.path.to_vec(),
    })
  }
}

#[napi(object)]
pub struct IndexOnMatchCallbackArgs {
  /// The path of entry.
  pub path: String,
  /// The patchspec that matched it.
  pub pathspec: String,
}

#[napi(object)]
pub struct IndexAddAllOptions {
  /// Files that are ignored will be skipped (unlike `addPath`). If a file is
  /// already tracked in the index, then it will be updated even if it is
  /// ignored. Pass the `force` flag to skip the checking of ignore rules.
  pub force: Option<bool>,
  /// The `pathspecs` are a list of file names or shell glob patterns that
  /// will matched against files in the repository's working directory. Each
  /// file that matches will be added to the index (either updating an
  /// existing entry or adding a new entry). You can disable glob expansion
  /// and force exact matching with the `disablePathspecMatch` flag.
  pub disable_pathspec_match: Option<bool>,
  /// To emulate `git add -A` and generate an error if the pathspec contains
  /// the exact path of an ignored file (when not using `force`), add the
  /// `checkPathspec` flag. This checks that each entry in `pathspecs`
  /// that is an exact match to a filename on disk is either not ignored or
  /// already in the index. If this check fails, the function will return
  /// an error.
  pub check_pathspec: Option<bool>,
  #[napi(ts_type = "(args: IndexOnMatchCallbackArgs) => number")]
  /// If you provide a callback function, it will be invoked on each matching
  /// item in the working directory immediately before it is added to /
  /// updated in the index. Returning zero will add the item to the index,
  /// greater than zero will skip the item, and less than zero will abort the
  /// scan an return an error to the caller.
  pub on_match: Option<Function<'static, IndexOnMatchCallbackArgs, i32>>,
}

impl IndexAddAllOptions {
  pub fn get_flag(&self) -> git2::IndexAddOption {
    let mut flag = git2::IndexAddOption::DEFAULT;
    if let Some(true) = self.force {
      flag |= git2::IndexAddOption::FORCE;
    }
    if let Some(true) = self.disable_pathspec_match {
      flag |= git2::IndexAddOption::DISABLE_PATHSPEC_MATCH;
    }
    if let Some(true) = self.check_pathspec {
      flag |= git2::IndexAddOption::CHECK_PATHSPEC;
    }
    flag
  }
}

#[napi(string_enum)]
pub enum IndexStage {
  /// Match any index stage.
  Any,
  /// A normal staged file in the index.
  Normal,
  /// The ancestor side of a conflict.
  Ancestor,
  /// The "ours" side of a conflict.
  Ours,
  /// The "theirs" side of a conflict.
  Theirs,
}

impl Default for IndexStage {
  fn default() -> Self {
    Self::Normal
  }
}

impl From<IndexStage> for i32 {
  fn from(value: IndexStage) -> Self {
    match value {
      IndexStage::Any => -1,
      IndexStage::Normal => 0,
      IndexStage::Ancestor => 1,
      IndexStage::Ours => 2,
      IndexStage::Theirs => 3,
    }
  }
}

#[napi(object)]
pub struct IndexRemoveOptions {
  pub stage: Option<IndexStage>,
}

#[napi(object)]
pub struct IndexRemoveAllOptions {
  #[napi(ts_type = "(args: IndexOnMatchCallbackArgs) => number")]
  /// If you provide a callback function, it will be invoked on each matching
  /// item in the index immediately before it is removed. Return 0 to remove
  /// the item, > 0 to skip the item, and < 0 to abort the scan.
  pub on_match: Option<Function<'static, IndexOnMatchCallbackArgs, i32>>,
}

#[napi(object)]
pub struct IndexUpdateAllOptions {
  #[napi(ts_type = "(args: IndexOnMatchCallbackArgs) => number")]
  /// If you provide a callback function, it will be invoked on each matching
  /// item in the index immediately before it is updated (either refreshed or
  /// removed depending on working directory state). Return 0 to proceed with
  /// updating the item, > 0 to skip the item, and < 0 to abort the scan.
  pub on_match: Option<Function<'static, IndexOnMatchCallbackArgs, i32>>,
}

#[napi]
/// A structure to represent a git [index][1]
///
/// [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct Index {
  pub(crate) inner: git2::Index,
}

#[napi]
impl Index {
  #[napi]
  /// Get index on-disk version.
  ///
  /// Valid return values are 2, 3, or 4.  If 3 is returned, an index
  /// with version 2 may be written instead, if the extension data in
  /// version 3 is not necessary.
  pub fn version(&self) -> u32 {
    self.inner.version()
  }

  #[napi]
  /// Set index on-disk version.
  ///
  /// Valid values are 2, 3, or 4.  If 2 is given, git_index_write may
  /// write an index with version 3 instead, if necessary to accurately
  /// represent the index.
  pub fn set_version(&mut self, version: u32) -> crate::Result<()> {
    self.inner.set_version(version)?;
    Ok(())
  }

  #[napi]
  /// Get one of the entries in the index by its path.
  pub fn get_by_path(&self, path: String, stage: Option<IndexStage>) -> Option<IndexEntry> {
    self
      .inner
      .get_path(Path::new(&path), stage.unwrap_or_default().into())
      .and_then(|x| IndexEntry::try_from(x).ok())
  }

  #[napi]
  /// Add or update an index entry from a file on disk
  ///
  /// The file path must be relative to the repository's working folder and
  /// must be readable.
  ///
  /// This method will fail in bare index instances.
  ///
  /// This forces the file to be added to the index, not looking at gitignore
  /// rules.
  ///
  /// If this file currently is the result of a merge conflict, this file will
  /// no longer be marked as conflicting. The data about the conflict will be
  /// moved to the "resolve undo" (REUC) section.
  pub fn add_path(&mut self, path: String) -> crate::Result<()> {
    self.inner.add_path(Path::new(&path))?;
    Ok(())
  }

  #[napi]
  /// Add or update index entries matching files in the working directory.
  ///
  /// This method will fail in bare index instances.
  ///
  /// The `pathspecs` are a list of file names or shell glob patterns that
  /// will matched against files in the repository's working directory. Each
  /// file that matches will be added to the index (either updating an
  /// existing entry or adding a new entry).
  ///
  /// @example
  ///
  /// Emulate `git add *`:
  ///
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const index = repo.index();
  /// index.addAll(['*']);
  /// index.write();
  /// ```
  pub fn add_all(
    &mut self,
    env: Env,
    pathspecs: Vec<String>,
    options: Option<IndexAddAllOptions>,
  ) -> crate::Result<()> {
    let (flag, callback) = match options {
      Some(opts) => {
        let flag = opts.get_flag();
        let callback = opts.on_match.and_then(|js_cb| js_cb.create_ref().ok()).map(|js_ref| {
          move |args: IndexOnMatchCallbackArgs| {
            js_ref
              .borrow_back(&env)
              .and_then(|callback| callback.call(args))
              .unwrap()
          }
        });
        (flag, callback)
      }
      None => (git2::IndexAddOption::DEFAULT, None),
    };
    if let Some(cb) = callback {
      let mut git2_cb = move |path: &Path, data: &[u8]| -> i32 {
        let path = path.to_string_lossy().to_string();
        let pathspec = std::str::from_utf8(data).unwrap().to_string();
        cb(IndexOnMatchCallbackArgs { path, pathspec })
      };
      self.inner.add_all(&pathspecs, flag, Some(&mut git2_cb))?;
    } else {
      self.inner.add_all(&pathspecs, flag, None)?;
    }
    Ok(())
  }

  #[napi]
  /// Update the contents of an existing index object in memory by reading
  /// from the hard disk.
  ///
  /// If force is true, this performs a "hard" read that discards in-memory
  /// changes and always reloads the on-disk index data. If there is no
  /// on-disk version, the index will be cleared.
  ///
  /// If force is false, this does a "soft" read that reloads the index data
  /// from disk only if it has changed since the last time it was loaded.
  /// Purely in-memory index data will be untouched. Be aware: if there are
  /// changes on disk, unwritten in-memory changes are discarded.
  pub fn read(&mut self, force: Option<bool>) -> crate::Result<()> {
    self.inner.read(force.unwrap_or_default())?;
    Ok(())
  }

  #[napi]
  /// Write the index as a tree.
  ///
  /// This method will scan the index and write a representation of its
  /// current state back to disk; it recursively creates tree objects for each
  /// of the subtrees stored in the index, but only returns the OID of the
  /// root tree. This is the OID that can be used e.g. to create a commit.
  ///
  /// The index instance cannot be bare, and needs to be associated to an
  /// existing repository.
  ///
  /// The index must not contain any file in conflict.
  pub fn write_tree(&mut self) -> crate::Result<String> {
    let id = self.inner.write_tree().map(|x| x.to_string())?;
    Ok(id)
  }

  #[napi]
  /// Remove an index entry corresponding to a file on disk.
  ///
  /// The file path must be relative to the repository's working folder. It
  /// may exist.
  ///
  /// If this file currently is the result of a merge conflict, this file will
  /// no longer be marked as conflicting. The data about the conflict will be
  /// moved to the "resolve undo" (REUC) section.
  pub fn remove_path(&mut self, path: String, options: Option<IndexRemoveOptions>) -> crate::Result<()> {
    if let Some(IndexRemoveOptions { stage: Some(stage) }) = options {
      self.inner.remove(Path::new(&path), i32::from(stage))?;
    } else {
      self.inner.remove_path(Path::new(&path))?;
    }
    Ok(())
  }

  #[napi]
  /// Remove all matching index entries.
  pub fn remove_all(
    &mut self,
    env: Env,
    pathspecs: Vec<String>,
    options: Option<IndexRemoveAllOptions>,
  ) -> crate::Result<()> {
    let callback = options
      .and_then(|x| x.on_match)
      .and_then(|x| x.create_ref().ok())
      .map(|js_ref| {
        move |args: IndexOnMatchCallbackArgs| {
          js_ref
            .borrow_back(&env)
            .and_then(|callback| callback.call(args))
            .unwrap()
        }
      });
    if let Some(cb) = callback {
      let mut git2_cb = move |path: &Path, data: &[u8]| -> i32 {
        let path = path.to_string_lossy().to_string();
        let pathspec = std::str::from_utf8(data).unwrap().to_string();
        cb(IndexOnMatchCallbackArgs { path, pathspec })
      };
      self.inner.remove_all(&pathspecs, Some(&mut git2_cb))?;
    } else {
      self.inner.remove_all(&pathspecs, None)?;
    }
    Ok(())
  }

  #[napi]
  /// Update all index entries to match the working directory
  ///
  /// This method will fail in bare index instances.
  ///
  /// This scans the existing index entries and synchronizes them with the
  /// working directory, deleting them if the corresponding working directory
  /// file no longer exists otherwise updating the information (including
  /// adding the latest version of file to the ODB if needed).
  pub fn update_all(
    &mut self,
    env: Env,
    pathspecs: Vec<String>,
    options: Option<IndexUpdateAllOptions>,
  ) -> crate::Result<()> {
    let callback = options
      .and_then(|x| x.on_match)
      .and_then(|x| x.create_ref().ok())
      .map(|js_ref| {
        move |args: IndexOnMatchCallbackArgs| {
          js_ref
            .borrow_back(&env)
            .and_then(|callback| callback.call(args))
            .unwrap()
        }
      });
    if let Some(cb) = callback {
      let mut git2_cb = move |path: &Path, data: &[u8]| -> i32 {
        let path = path.to_string_lossy().to_string();
        let pathspec = std::str::from_utf8(data).unwrap().to_string();
        cb(IndexOnMatchCallbackArgs { path, pathspec })
      };
      self.inner.update_all(&pathspecs, Some(&mut git2_cb))?;
    } else {
      self.inner.update_all(&pathspecs, None)?;
    }
    Ok(())
  }

  #[napi]
  /// Get the count of entries currently in the index
  pub fn count(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  /// Return `true` is there is no entry in the index
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Get the full path to the index file on disk.
  ///
  /// Returns `None` if this is an in-memory index.
  pub fn path(&self, env: Env) -> Option<JsString> {
    self.inner.path().and_then(|x| util::path_to_js_string(&env, x).ok())
  }

  #[napi]
  /// Does this index have conflicts?
  ///
  /// Returns `true` if the index contains conflicts, `false` if it does not.
  pub fn has_conflicts(&self) -> bool {
    self.inner.has_conflicts()
  }

  #[napi]
  /// Get an iterator over the entries in this index.
  pub fn entries(&self, this: Reference<Index>, env: Env) -> crate::Result<IndexEntries> {
    let inner = this.share_with(env, |index| Ok(index.inner.iter()))?;
    Ok(IndexEntries { inner })
  }
}

#[napi(iterator)]
/// An iterator over the entries in an index
pub struct IndexEntries {
  pub(crate) inner: SharedReference<Index, git2::IndexEntries<'static>>,
}

#[napi]
impl Generator for IndexEntries {
  type Yield = IndexEntry;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().and_then(|x| IndexEntry::try_from(x).ok())
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Get the Index file for this repository.
  ///
  /// If a custom index has not been set, the default index for the repository
  /// will be returned (the one located in .git/index).
  ///
  /// **Caution**: If the [`git2::Repository`] of this index is dropped, then this
  /// [`git2::Index`] will become detached, and most methods on it will fail. See
  /// [`git2::Index::open`]. Be sure the repository has a binding such as a local
  /// variable to keep it alive at least as long as the index.
  pub fn index(&self) -> crate::Result<Index> {
    Ok(Index {
      inner: self.inner.index()?,
    })
  }
}
