use crate::diff::DiffDelta;
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
pub struct Status {
  pub current: bool,
  pub index_new: bool,
  pub index_modified: bool,
  pub index_deleted: bool,
  pub index_renamed: bool,
  pub index_typechange: bool,
  pub wt_new: bool,
  pub wt_modified: bool,
  pub wt_deleted: bool,
  pub wt_typechange: bool,
  pub wt_renamed: bool,
  // TODO: Field does not exist on current `git2-rs` version.
  // Wait for updates (https://github.com/rust-lang/git2-rs/pull/1151)
  // pub wt_unreadable: bool,
  pub ignored: bool,
  pub conflicted: bool,
}

impl From<git2::Status> for Status {
  fn from(value: git2::Status) -> Self {
    let current = value.contains(git2::Status::CURRENT);
    let index_new = value.contains(git2::Status::INDEX_NEW);
    let index_modified = value.contains(git2::Status::INDEX_MODIFIED);
    let index_deleted = value.contains(git2::Status::INDEX_DELETED);
    let index_renamed = value.contains(git2::Status::INDEX_RENAMED);
    let index_typechange = value.contains(git2::Status::INDEX_TYPECHANGE);
    let wt_new = value.contains(git2::Status::WT_NEW);
    let wt_modified = value.contains(git2::Status::WT_MODIFIED);
    let wt_deleted = value.contains(git2::Status::WT_DELETED);
    let wt_typechange = value.contains(git2::Status::WT_TYPECHANGE);
    let wt_renamed = value.contains(git2::Status::WT_RENAMED);
    let ignored = value.contains(git2::Status::IGNORED);
    let conflicted = value.contains(git2::Status::CONFLICTED);
    Self {
      current,
      index_new,
      index_modified,
      index_deleted,
      index_renamed,
      index_typechange,
      wt_new,
      wt_modified,
      wt_deleted,
      wt_typechange,
      wt_renamed,
      ignored,
      conflicted,
    }
  }
}

#[napi(string_enum)]
/// - `Index` : Only gives status based on HEAD to index comparison, not looking at
/// working directory changes.
/// - `Workdir` : Only gives status based on index to working directory comparison, not
/// comparing the index to the HEAD.
/// - `IndexAndWorkdi` : The default, this roughly matches `git status --porcelain` regarding
/// which files are included and in what order.
pub enum StatusShow {
  Index,
  Workdir,
  IndexAndWorkdir,
}

impl From<StatusShow> for git2::StatusShow {
  fn from(value: StatusShow) -> git2::StatusShow {
    match value {
      StatusShow::Index => git2::StatusShow::Index,
      StatusShow::Workdir => git2::StatusShow::Workdir,
      StatusShow::IndexAndWorkdir => git2::StatusShow::IndexAndWorkdir,
    }
  }
}

#[napi(object)]
pub struct StatusOptions {
  /// Select the files on which to report status.
  /// The default, if unspecified, is to show the index and the working directory.
  pub show: Option<StatusShow>,
  /// Path patterns to match (using fnmatch-style matching).
  /// If the `disablePathspecMatch` option is given, then this is a literal
  /// path to match. If this is not called, then there will be no patterns to
  /// match and the entire directory will be used.
  pub pathspecs: Option<Vec<String>>,
  /// Flag whether untracked files will be included.
  /// Untracked files will only be included if the workdir files are included
  /// in the status "show" option.
  pub include_untracked: Option<bool>,
  pub include_ignored: Option<bool>,
  pub include_unmodified: Option<bool>,
  pub exclude_submodules: Option<bool>,
  pub recurse_untracked_dirs: Option<bool>,
  pub disable_pathspec_match: Option<bool>,
  pub recurse_ignored_dirs: Option<bool>,
  pub renames_head_to_index: Option<bool>,
  pub renames_index_to_workdir: Option<bool>,
  pub sort_case_sensitively: Option<bool>,
  pub sort_case_insensitively: Option<bool>,
  pub renames_from_rewrites: Option<bool>,
  pub no_refresh: Option<bool>,
  pub update_index: Option<bool>,
  pub include_unreadable: Option<bool>,
  pub include_unreadable_as_untracked: Option<bool>,
  pub rename_threshold: Option<u16>,
}

impl From<StatusOptions> for git2::StatusOptions {
  fn from(value: StatusOptions) -> Self {
    let mut opts = git2::StatusOptions::new();
    if let Some(show) = value.show {
      opts.show(show.into());
    }
    if let Some(pathspecs) = value.pathspecs {
      for pathspec in pathspecs {
        opts.pathspec(&pathspec);
      }
    }
    if let Some(include_untracked) = value.include_untracked {
      opts.include_untracked(include_untracked);
    }
    if let Some(include_ignored) = value.include_ignored {
      opts.include_ignored(include_ignored);
    }
    if let Some(include_unmodified) = value.include_unmodified {
      opts.include_unmodified(include_unmodified);
    }
    if let Some(exclude_submodules) = value.exclude_submodules {
      opts.exclude_submodules(exclude_submodules);
    }
    if let Some(recurse_untracked_dirs) = value.recurse_untracked_dirs {
      opts.recurse_untracked_dirs(recurse_untracked_dirs);
    }
    if let Some(disable_pathspec_match) = value.disable_pathspec_match {
      opts.disable_pathspec_match(disable_pathspec_match);
    }
    if let Some(recurse_ignored_dirs) = value.recurse_ignored_dirs {
      opts.recurse_ignored_dirs(recurse_ignored_dirs);
    }
    if let Some(renames_head_to_index) = value.renames_head_to_index {
      opts.renames_head_to_index(renames_head_to_index);
    }
    if let Some(renames_index_to_workdir) = value.renames_index_to_workdir {
      opts.renames_index_to_workdir(renames_index_to_workdir);
    }
    if let Some(sort_case_sensitively) = value.sort_case_sensitively {
      opts.sort_case_sensitively(sort_case_sensitively);
    }
    if let Some(sort_case_insensitively) = value.sort_case_insensitively {
      opts.sort_case_insensitively(sort_case_insensitively);
    }
    if let Some(renames_from_rewrites) = value.renames_from_rewrites {
      opts.renames_from_rewrites(renames_from_rewrites);
    }
    if let Some(no_refresh) = value.no_refresh {
      opts.no_refresh(no_refresh);
    }
    if let Some(update_index) = value.update_index {
      opts.update_index(update_index);
    }
    if let Some(include_unreadable) = value.include_unreadable {
      opts.include_unreadable(include_unreadable);
    }
    if let Some(include_unreadable_as_untracked) = value.include_unreadable_as_untracked {
      opts.include_unreadable_as_untracked(include_unreadable_as_untracked);
    }
    if let Some(rename_threshold) = value.rename_threshold {
      opts.rename_threshold(rename_threshold);
    }
    opts
  }
}

#[napi]
/// A container for a list of status information about a repository.
///
/// Each instance appears as if it were a collection, having a length and
/// allowing indexing, as well as providing an iterator.
pub struct Statuses {
  pub(crate) inner: SharedReference<Repository, git2::Statuses<'static>>,
}

#[napi]
impl Statuses {
  #[napi]
  /// Gets a status entry from this list at the specified index.
  ///
  /// @category Status/Statuses
  /// @signature
  /// ```ts
  /// class Statuses {
  ///   get(index: number): StatusEntry | null;
  /// }
  /// ```
  ///
  /// @param {number} index -
  /// @returns A status entry from this list at the specified index. Returns `null` if the status
  /// entry does not exist.
  pub fn get(&self, index: u32, this: Reference<Statuses>, env: Env) -> Option<StatusEntry> {
    this
      .share_with(env, move |repo| {
        repo
          .inner
          .get(index as usize)
          .ok_or(Error::new(napi::Status::GenericFailure, "not found"))
      })
      .ok()
      .map(|inner| StatusEntry { inner })
  }

  #[napi]
  /// Gets the count of status entries in this list.
  ///
  /// @category Status/Statuses
  /// @signature
  /// ```ts
  /// class Statuses {
  ///   len(): number;
  /// }
  /// ```
  ///
  /// @returns If there are no changes in status (according to the options given
  /// when the status list was created), this should return 0.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  #[napi]
  /// @category Status/Statuses
  /// @signature
  /// ```ts
  /// class Statuses {
  ///   isEmpty(): boolean;
  /// }
  /// ```
  ///
  /// @returns Return `true` if there is no status entry in this list.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }
}

#[napi]
pub struct StatusEntry {
  pub(crate) inner: SharedReference<Statuses, git2::StatusEntry<'static>>,
}

#[napi]
impl StatusEntry {
  #[napi]
  /// Access this entry's path name as a string.
  ///
  /// @category Status/StatusEntry
  /// @signature
  /// ```ts
  /// class StatusEntry {
  ///   path(): string;
  /// }
  /// ```
  pub fn path(&self) -> crate::Result<String> {
    let path = std::str::from_utf8(self.inner.path_bytes())?;
    Ok(path.to_string())
  }

  #[napi]
  /// Access the status for this file.
  ///
  /// @category Status/StatusEntry
  /// @signature
  /// ```ts
  /// class StatusEntry {
  ///   status(): Status;
  /// }
  /// ```
  pub fn status(&self) -> Status {
    Status::from(self.inner.status())
  }

  #[napi]
  /// Access detailed information about the differences between the file in
  /// HEAD and the file in the index.
  ///
  /// @category Status/StatusEntry
  /// @signature
  /// ```ts
  /// class StatusEntry {
  ///   headToIndex(): DiffDelta | null;
  /// }
  /// ```
  ///
  /// @returns todo
  pub fn head_to_index(&self) -> Option<DiffDelta> {
    self.inner.head_to_index().map(|inner| DiffDelta { inner })
  }

  #[napi]
  /// Access detailed information about the differences between the file in
  /// the index and the file in the working directory.
  ///
  /// @category Status/StatusEntry
  /// @signature
  /// ```ts
  /// class StatusEntry {
  ///   indexToWorkdir(): DiffDelta | null;
  /// }
  /// ```
  ///
  /// @returns todo
  pub fn index_to_workdir(&self) -> Option<DiffDelta> {
    self.inner.index_to_workdir().map(|inner| DiffDelta { inner })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Test if the ignore rules apply to a given file.
  ///
  /// This function checks the ignore rules to see if they would apply to the
  /// given file. This indicates if the file would be ignored regardless of
  /// whether the file is already in the index or committed to the repository.
  ///
  /// One way to think of this is if you were to do "git add ." on the
  /// directory containing the file, would it be added or not?
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   statusShouldIgnore(path: string): boolean;
  /// }
  /// ```
  ///
  /// @param {string} path - A given file path.
  /// @returns Returns `true` if the ignore rules apply to a given file.
  pub fn status_should_ignore(&self, path: String) -> crate::Result<bool> {
    Ok(self.inner.status_should_ignore(Path::new(&path))?)
  }

  #[napi]
  /// Get file status for a single file.
  ///
  /// This tries to get status for the filename that you give. If no files
  /// match that name (in either the HEAD, index, or working directory), this
  /// returns NotFound.
  ///
  /// If the name matches multiple files (for example, if the path names a
  /// directory or if running on a case- insensitive filesystem and yet the
  /// HEAD has two entries that both match the path), then this returns
  /// Ambiguous because it cannot give correct results.
  ///
  /// This does not do any sort of rename detection. Renames require a set of
  /// targets and because of the path filtering, there is not enough
  /// information to check renames correctly. To check file status with rename
  /// detection, there is no choice but to do a full `statuses` and scan
  /// through looking for the path that you are interested in.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getStatusFile(path: string): Status;
  /// }
  /// ```
  ///
  /// @param {string} path - A single file path.
  /// @returns The `Status` of this single file.
  /// @throws Throws error if the status file does not exist.
  pub fn get_status_file(&self, path: String) -> crate::Result<Status> {
    let status: Status = self.inner.status_file(Path::new(&path))?.into();
    Ok(status)
  }

  #[napi]
  /// Get file status for a single file.
  ///
  /// This tries to get status for the filename that you give. If no files
  /// match that name (in either the HEAD, index, or working directory), this
  /// returns NotFound.
  ///
  /// If the name matches multiple files (for example, if the path names a
  /// directory or if running on a case- insensitive filesystem and yet the
  /// HEAD has two entries that both match the path), then this returns
  /// Ambiguous because it cannot give correct results.
  ///
  /// This does not do any sort of rename detection. Renames require a set of
  /// targets and because of the path filtering, there is not enough
  /// information to check renames correctly. To check file status with rename
  /// detection, there is no choice but to do a full `statuses` and scan
  /// through looking for the path that you are interested in.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findStatusFile(path: string): Status | null;
  /// }
  /// ```
  ///
  /// @param {string} path - A single file path.
  /// @returns The `Status` of this single file. If the status file does not exists, returns `null`.
  pub fn find_status_file(&self, path: String) -> Option<Status> {
    self.get_status_file(path).ok()
  }

  #[napi]
  /// Gather file status information and populate the returned structure.
  ///
  /// Note that if a pathspec is given in the options to filter the
  /// status, then the results from rename detection (if you enable it) may
  /// not be accurate. To do rename detection properly, this must be called
  /// with no pathspec so that all files can be considered.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   statuses(): Statuses;
  /// }
  /// ```
  ///
  /// @returns A container for a list of status information about a repository.
  pub fn statuses(&self, this: Reference<Repository>, env: Env) -> crate::Result<Statuses> {
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .statuses(None)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Statuses { inner })
  }
}
