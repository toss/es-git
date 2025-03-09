use crate::index::Index;
use crate::repository::Repository;
use crate::tree::Tree;
use crate::util::{bitflags_contain, path_to_js_string};
use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;
use std::ops::Deref;

#[napi]
#[repr(u32)]
pub enum DiffFlags {
  /// File(s) treated as binary data.
  Binary = 1,
  /// File(s) treated as text data.
  NotBinary = 2,
  /// `id` value is known correct.
  ValidId = 4,
  /// File exists at this side of the delta.
  Exists = 8,
}

#[napi]
/// Check diff flags contains given flags.
pub fn diff_flags_contains(source: u32, target: u32) -> bool {
  bitflags_contain(
    git2::DiffFlags::from_bits_retain(source),
    git2::DiffFlags::from_bits_retain(target),
  )
}

#[napi(string_enum)]
/// What type of change is described by a `DiffDelta`?
pub enum DeltaType {
  /// No changes
  Unmodified,
  /// Entry does not exist in an old version
  Added,
  /// Entry does not exist in a new version
  Deleted,
  /// Entry content changed between old and new
  Modified,
  /// Entry was renamed between old and new
  Renamed,
  /// Entry was copied from another old entry
  Copied,
  /// Entry is ignored item in workdir
  Ignored,
  /// Entry is untracked item in workdir
  Untracked,
  /// Type of entry changed between old and new
  Typechange,
  /// Entry is unreadable
  Unreadable,
  /// Entry in the index is conflicted
  Conflicted,
}

impl From<git2::Delta> for DeltaType {
  fn from(value: git2::Delta) -> Self {
    match value {
      git2::Delta::Unmodified => DeltaType::Unmodified,
      git2::Delta::Added => DeltaType::Added,
      git2::Delta::Deleted => DeltaType::Deleted,
      git2::Delta::Modified => DeltaType::Modified,
      git2::Delta::Renamed => DeltaType::Renamed,
      git2::Delta::Copied => DeltaType::Copied,
      git2::Delta::Ignored => DeltaType::Ignored,
      git2::Delta::Untracked => DeltaType::Untracked,
      git2::Delta::Typechange => DeltaType::Typechange,
      git2::Delta::Unreadable => DeltaType::Unreadable,
      git2::Delta::Conflicted => DeltaType::Conflicted,
    }
  }
}

impl From<DeltaType> for git2::Delta {
  fn from(value: DeltaType) -> Self {
    match value {
      DeltaType::Unmodified => git2::Delta::Unmodified,
      DeltaType::Added => git2::Delta::Added,
      DeltaType::Deleted => git2::Delta::Deleted,
      DeltaType::Modified => git2::Delta::Modified,
      DeltaType::Renamed => git2::Delta::Renamed,
      DeltaType::Copied => git2::Delta::Copied,
      DeltaType::Ignored => git2::Delta::Ignored,
      DeltaType::Untracked => git2::Delta::Untracked,
      DeltaType::Typechange => git2::Delta::Typechange,
      DeltaType::Unreadable => git2::Delta::Unreadable,
      DeltaType::Conflicted => git2::Delta::Conflicted,
    }
  }
}

#[napi(string_enum)]
/// Possible output formats for diff data.
pub enum DiffFormat {
  /// full `git diff` (default)
  Patch,
  /// just the headers of the patch
  PatchHeader,
  /// like `git diff --raw`
  Raw,
  /// like `git diff --name-only`
  NameOnly,
  /// like `git diff --name-status`
  NameStatus,
  /// `git diff` as used by `git patch-id`
  PatchId,
}

impl Default for DiffFormat {
  fn default() -> Self {
    Self::Patch
  }
}

impl From<DiffFormat> for git2::DiffFormat {
  fn from(value: DiffFormat) -> Self {
    match value {
      DiffFormat::Patch => git2::DiffFormat::Patch,
      DiffFormat::PatchHeader => git2::DiffFormat::PatchHeader,
      DiffFormat::Raw => git2::DiffFormat::Raw,
      DiffFormat::NameOnly => git2::DiffFormat::NameOnly,
      DiffFormat::NameStatus => git2::DiffFormat::NameStatus,
      DiffFormat::PatchId => git2::DiffFormat::PatchId,
    }
  }
}

#[napi(object)]
pub struct DiffPrintOptions {
  pub format: Option<DiffFormat>,
}

#[napi]
/// The diff object that contains all individual file deltas.
///
/// This is an opaque structure which will be allocated by one of the diff
/// generator functions on the `Repository` class (e.g. `diffTreeToTree`
/// or other `diff*` functions).
///
/// @hideconstructor
pub struct Diff {
  pub(crate) inner: SharedReference<Repository, git2::Diff<'static>>,
}

#[napi]
impl Diff {
  #[napi]
  /// Merge one diff into another.
  ///
  /// This merges items from the "from" list into the "self" list.  The
  /// resulting diff will have all items that appear in either list.
  /// If an item appears in both lists, then it will be "merged" to appear
  /// as if the old version was from the "onto" list and the new version
  /// is from the "from" list (with the exception that if the item has a
  /// pending DELETE in the middle, then it will show as deleted).
  pub fn merge(&mut self, diff: &Diff) -> crate::Result<()> {
    self.inner.merge(diff.inner.deref())?;
    Ok(())
  }

  #[napi]
  /// Returns an iterator over the deltas in this diff.
  pub fn deltas(&self, env: Env, this: Reference<Diff>) -> crate::Result<Deltas> {
    Ok(Deltas {
      inner: this.share_with(env, |diff| Ok(diff.inner.deltas()))?,
    })
  }

  #[napi]
  /// Check if deltas are sorted case sensitively or insensitively.
  pub fn is_sorted_icase(&self) -> bool {
    self.inner.is_sorted_icase()
  }

  #[napi]
  /// Accumulate diff statistics for all patches.
  pub fn stats(&self) -> crate::Result<DiffStats> {
    Ok(DiffStats {
      inner: self.inner.stats()?,
    })
  }

  #[napi]
  /// Iterate over a diff generating formatted text output.
  pub fn print(&self, options: Option<DiffPrintOptions>) -> String {
    let format = options.and_then(|x| x.format).unwrap_or_default();
    let mut lines: Vec<String> = vec![];
    let _ = self.inner.print(format.into(), |_delta, _hunk, line| {
      if let Ok(content) = std::str::from_utf8(line.content()) {
        lines.push(content.to_string());
      }
      true
    });
    lines.join("")
  }
}

#[napi]
/// A class describing a hunk of a diff.
///
/// @hideconstructor
pub struct DiffStats {
  pub(crate) inner: git2::DiffStats,
}

#[napi]
impl DiffStats {
  #[napi(getter)]
  /// Get the total number of files changed in a diff.
  pub fn files_changed(&self) -> usize {
    self.inner.files_changed()
  }

  #[napi(getter)]
  /// Get the total number of insertions in a diff
  pub fn insertions(&self) -> usize {
    self.inner.insertions()
  }

  #[napi(getter)]
  /// Get the total number of deletions in a diff
  pub fn deletions(&self) -> usize {
    self.inner.deletions()
  }
}

#[napi(iterator)]
/// An iterator over the diffs in a delta.
///
/// @hideconstructor
pub struct Deltas {
  pub(crate) inner: SharedReference<Diff, git2::Deltas<'static>>,
}

#[napi]
impl Generator for Deltas {
  type Yield = DiffDelta;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().map(|x| DiffDelta { inner: x })
  }
}

#[napi]
/// Description of changes to one entry.
///
/// @hideconstructor
pub struct DiffDelta {
  pub(crate) inner: git2::DiffDelta<'static>,
}

#[napi]
impl DiffDelta {
  #[napi]
  /// Returns the flags on the delta.
  ///
  /// For more information, see `DiffFlags`'s documentation.
  pub fn flags(&self) -> u32 {
    self.inner.flags().bits()
  }

  #[napi]
  /// Returns the number of files in this delta.
  pub fn num_files(&self) -> u32 {
    self.inner.nfiles() as u32
  }

  #[napi]
  /// Returns the status of this entry.
  pub fn status(&self) -> DeltaType {
    self.inner.status().into()
  }

  #[napi]
  /// Return the file which represents the "from" side of the diff.
  ///
  /// What side this means depends on the function that was used to generate
  /// the diff and will be documented on the function itself.
  pub fn old_file(&self) -> DiffFile {
    DiffFile {
      inner: self.inner.old_file(),
    }
  }

  #[napi]
  /// Return the file which represents the "to" side of the diff.
  ///
  /// What side this means depends on the function that was used to generate
  /// the diff and will be documented on the function itself.
  pub fn new_file(&self) -> DiffFile {
    DiffFile {
      inner: self.inner.new_file(),
    }
  }
}

#[napi(string_enum)]
/// Valid modes for index and tree entries.
pub enum FileMode {
  Unreadable,
  Tree,
  Blob,
  /// Group writable blob. Obsolete mode kept for compatibility reasons
  BlobGroupWritable,
  BlobExecutable,
  Link,
  Commit,
}

impl From<git2::FileMode> for FileMode {
  fn from(value: git2::FileMode) -> Self {
    match value {
      git2::FileMode::Unreadable => FileMode::Unreadable,
      git2::FileMode::Tree => FileMode::Tree,
      git2::FileMode::Blob => FileMode::Blob,
      git2::FileMode::BlobGroupWritable => FileMode::BlobGroupWritable,
      git2::FileMode::BlobExecutable => FileMode::BlobExecutable,
      git2::FileMode::Link => FileMode::Link,
      git2::FileMode::Commit => FileMode::Commit,
    }
  }
}

impl From<FileMode> for git2::FileMode {
  fn from(value: FileMode) -> Self {
    match value {
      FileMode::Unreadable => git2::FileMode::Unreadable,
      FileMode::Tree => git2::FileMode::Tree,
      FileMode::Blob => git2::FileMode::Blob,
      FileMode::BlobGroupWritable => git2::FileMode::BlobGroupWritable,
      FileMode::BlobExecutable => git2::FileMode::BlobExecutable,
      FileMode::Link => git2::FileMode::Link,
      FileMode::Commit => git2::FileMode::Commit,
    }
  }
}

#[napi]
/// Description of one side of a delta.
///
/// Although this is called a "file" it could represent a file, a symbolic
/// link, a submodule commit id, or even a tree (although that only happens if
/// you are tracking type changes or ignored/untracked directories).
///
/// @hideconstructor
pub struct DiffFile {
  pub(crate) inner: git2::DiffFile<'static>,
}

#[napi]
impl DiffFile {
  #[napi]
  /// Returns the Oid of this item.
  ///
  /// If this entry represents an absent side of a diff (e.g. the `oldFile`
  /// of a `Added` delta), then the oid returned will be zeroes.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Returns the path of the entry relative to the working directory of the
  /// repository.
  pub fn path(&self, env: Env) -> Option<JsString> {
    self.inner.path().and_then(|p| path_to_js_string(&env, p).ok())
  }

  #[napi]
  /// Returns the size of this entry, in bytes.
  pub fn size(&self) -> u64 {
    self.inner.size()
  }

  #[napi]
  /// Returns `true` if file(s) are treated as binary data.
  pub fn is_binary(&self) -> bool {
    self.inner.is_binary()
  }

  #[napi]
  /// Returns `true` if `id` value is known correct.
  pub fn is_valid_id(&self) -> bool {
    self.inner.is_valid_id()
  }

  #[napi]
  /// Returns `true` if file exists at this side of the delta.
  pub fn exists(&self) -> bool {
    self.inner.exists()
  }

  #[napi]
  /// Returns file mode.
  pub fn mode(&self) -> FileMode {
    self.inner.mode().into()
  }
}

#[napi(object)]
/// Describing options about how the diff should be executed.
pub struct DiffOptions {
  /// Flag indicating whether the sides of the diff will be reversed.
  pub reverse: Option<bool>,
  /// Flag indicating whether ignored files are included.
  pub include_ignored: Option<bool>,
  /// Flag indicating whether ignored directories are traversed deeply or not.
  pub recurse_ignored_dirs: Option<bool>,
  /// Flag indicating whether untracked files are in the diff
  pub include_untracked: Option<bool>,
  /// Flag indicating whether untracked directories are traversed deeply or
  /// not.
  pub recurse_untracked_dirs: Option<bool>,
  /// Flag indicating whether unmodified files are in the diff.
  pub include_unmodified: Option<bool>,
  /// If enabled, then Typechange delta records are generated.
  pub include_typechange: Option<bool>,
  /// Event with `includeTypechange`, the tree returned generally shows a
  /// deleted blob. This flag correctly labels the tree transitions as a
  /// typechange record with the `new_file`'s mode set to tree.
  ///
  /// Note that the tree SHA will not be available.
  pub include_typechange_trees: Option<bool>,
  /// Flag indicating whether file mode changes are ignored.
  pub ignore_filemode: Option<bool>,
  /// Flag indicating whether all submodules should be treated as unmodified.
  pub ignore_submodules: Option<bool>,
  /// Flag indicating whether case insensitive filenames should be used.
  pub ignore_case: Option<bool>,
  /// If pathspecs are specified, this flag means that they should be applied
  /// as an exact match instead of a fnmatch pattern.
  pub disable_pathspec_match: Option<bool>,
  /// Disable updating the `binary` flag in delta records. This is useful when
  /// iterating over a diff if you don't need hunk and data callbacks and want
  /// to avoid having to load a file completely.
  pub skip_binary_check: Option<bool>,
  /// When diff finds an untracked directory, to match the behavior of core
  /// Git, it scans the contents for ignored and untracked files. If all
  /// contents are ignored, then the directory is ignored; if any contents are
  /// not ignored, then the directory is untracked. This is extra work that
  /// may not matter in many cases.
  ///
  /// This flag turns off that scan and immediately labels an untracked
  /// directory as untracked (changing the behavior to not match core git).
  pub enable_fast_untracked_dirs: Option<bool>,
  /// When diff finds a file in the working directory with stat information
  /// different from the index, but the OID ends up being the same, write the
  /// correct stat information into the index. Note: without this flag, diff
  /// will always leave the index untouched.
  pub update_index: Option<bool>,
  /// Include unreadable files in the diff
  pub include_unreadable: Option<bool>,
  /// Include unreadable files in the diff as untracked files
  pub include_unreadable_as_untracked: Option<bool>,
  /// Treat all files as text, disabling binary attributes and detection.
  pub force_text: Option<bool>,
  /// Treat all files as binary, disabling text diffs
  pub force_binary: Option<bool>,
  /// Ignore all whitespace
  pub ignore_whitespace: Option<bool>,
  /// Ignore changes in the amount of whitespace
  pub ignore_whitespace_change: Option<bool>,
  /// Ignore whitespace at the end of line
  pub ignore_whitespace_eol: Option<bool>,
  /// Ignore blank lines
  pub ignore_blank_lines: Option<bool>,
  /// When generating patch text, include the content of untracked files.
  ///
  /// This automatically turns on `includeUntracked` but it does not turn on
  /// `recurseUntrackedDirs`. Add that flag if you want the content of every
  /// single untracked file.
  pub show_untracked_content: Option<bool>,
  /// When generating output, include the names of unmodified files if they
  /// are included in the `Diff`. Normally these are skipped in the formats
  /// that list files (e.g. name-only, name-status, raw). Even with this these
  /// will not be included in the patch format.
  pub show_unmodified: Option<bool>,
  /// Use the "patience diff" algorithm
  pub patience: Option<bool>,
  /// Take extra time to find the minimal diff
  pub minimal: Option<bool>,
  /// Include the necessary deflate/delta information so that `git-apply` can
  /// apply given diff information to binary files.
  pub show_binary: Option<bool>,
  /// Use a heuristic that takes indentation and whitespace into account
  /// which generally can produce better diffs when dealing with ambiguous
  /// diff hunks.
  pub indent_heuristic: Option<bool>,
  /// Set the number of unchanged lines that define the boundary of a hunk
  /// (and to display before and after).
  ///
  /// The default value for this is 3.
  pub context_lines: Option<u32>,
  /// Set the maximum number of unchanged lines between hunk boundaries before
  /// the hunks will be merged into one.
  ///
  /// The default value for this is 0.
  pub interhunk_lines: Option<u32>,
  /// The default value for this is `core.abbrev` or 7 if unset.
  pub id_abbrev: Option<u16>,
  /// Maximum size (in bytes) above which a blob will be marked as binary
  /// automatically.
  ///
  /// A negative value will disable this entirely.
  ///
  /// The default value for this is 512MB.
  pub max_size: Option<i64>,
  /// The virtual "directory" to prefix old file names with in hunk headers.
  ///
  /// The default value for this is "a".
  pub old_prefix: Option<String>,
  /// The virtual "directory" to prefix new file names with in hunk headers.
  ///
  /// The default value for this is "b".
  pub new_prefix: Option<String>,
  /// Add to the array of paths/fnmatch patterns to constrain the diff.
  pub pathspecs: Option<Vec<String>>,
}

impl From<DiffOptions> for git2::DiffOptions {
  fn from(value: DiffOptions) -> Self {
    let mut opts = git2::DiffOptions::new();
    if let Some(reverse) = value.reverse {
      opts.reverse(reverse);
    }
    if let Some(include_ignored) = value.include_ignored {
      opts.include_ignored(include_ignored);
    }
    if let Some(recurse_ignored_dirs) = value.recurse_ignored_dirs {
      opts.recurse_ignored_dirs(recurse_ignored_dirs);
    }
    if let Some(include_untracked) = value.include_untracked {
      opts.include_untracked(include_untracked);
    }
    if let Some(recurse_untracked_dirs) = value.recurse_untracked_dirs {
      opts.recurse_untracked_dirs(recurse_untracked_dirs);
    }
    if let Some(include_unmodified) = value.include_unmodified {
      opts.include_unmodified(include_unmodified);
    }
    if let Some(include_typechange) = value.include_typechange {
      opts.include_typechange(include_typechange);
    }
    if let Some(include_typechange_trees) = value.include_typechange_trees {
      opts.include_typechange_trees(include_typechange_trees);
    }
    if let Some(ignore_filemode) = value.ignore_filemode {
      opts.ignore_filemode(ignore_filemode);
    }
    if let Some(ignore_submodules) = value.ignore_submodules {
      opts.ignore_submodules(ignore_submodules);
    }
    if let Some(ignore_case) = value.ignore_case {
      opts.ignore_case(ignore_case);
    }
    if let Some(disable_pathspec_match) = value.disable_pathspec_match {
      opts.disable_pathspec_match(disable_pathspec_match);
    }
    if let Some(skip_binary_check) = value.skip_binary_check {
      opts.skip_binary_check(skip_binary_check);
    }
    if let Some(enable_fast_untracked_dirs) = value.enable_fast_untracked_dirs {
      opts.enable_fast_untracked_dirs(enable_fast_untracked_dirs);
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
    if let Some(force_text) = value.force_text {
      opts.force_text(force_text);
    }
    if let Some(force_binary) = value.force_binary {
      opts.force_binary(force_binary);
    }
    if let Some(ignore_whitespace) = value.ignore_whitespace {
      opts.ignore_whitespace(ignore_whitespace);
    }
    if let Some(ignore_whitespace_change) = value.ignore_whitespace_change {
      opts.ignore_whitespace_change(ignore_whitespace_change);
    }
    if let Some(ignore_whitespace_eol) = value.ignore_whitespace_eol {
      opts.ignore_whitespace_eol(ignore_whitespace_eol);
    }
    if let Some(ignore_blank_lines) = value.ignore_blank_lines {
      opts.ignore_blank_lines(ignore_blank_lines);
    }
    if let Some(show_untracked_content) = value.show_untracked_content {
      opts.show_untracked_content(show_untracked_content);
    }
    if let Some(show_unmodified) = value.show_unmodified {
      opts.show_unmodified(show_unmodified);
    }
    if let Some(patience) = value.patience {
      opts.patience(patience);
    }
    if let Some(minimal) = value.minimal {
      opts.minimal(minimal);
    }
    if let Some(show_binary) = value.show_binary {
      opts.show_binary(show_binary);
    }
    if let Some(indent_heuristic) = value.indent_heuristic {
      opts.indent_heuristic(indent_heuristic);
    }
    if let Some(context_lines) = value.context_lines {
      opts.context_lines(context_lines);
    }
    if let Some(interhunk_lines) = value.interhunk_lines {
      opts.interhunk_lines(interhunk_lines);
    }
    if let Some(id_abbrev) = value.id_abbrev {
      opts.id_abbrev(id_abbrev);
    }
    if let Some(max_size) = value.max_size {
      opts.max_size(max_size);
    }
    if let Some(old_prefix) = value.old_prefix {
      opts.old_prefix(old_prefix);
    }
    if let Some(new_prefix) = value.new_prefix {
      opts.new_prefix(new_prefix);
    }
    if let Some(pathspecs) = value.pathspecs {
      for pathspec in pathspecs {
        opts.pathspec(pathspec);
      }
    }
    opts
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Create a diff with the difference between two tree objects.
  ///
  /// This is equivalent to `git diff <old-tree> <new-tree>`.
  ///
  /// The first tree will be used for the "oldFile" side of the delta and the
  /// second tree will be used for the "newFile" side of the delta. You can
  /// pass `null` to indicate an empty tree, although it is an error to pass
  /// `null` for both the `oldTree` and `newTree`.
  pub fn diff_tree_to_tree(
    &self,
    this: Reference<Repository>,
    env: Env,
    old_tree: Option<&Tree>,
    new_tree: Option<&Tree>,
    options: Option<DiffOptions>,
  ) -> crate::Result<Diff> {
    let mut opts: git2::DiffOptions = options.map(|x| x.into()).unwrap_or_default();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .diff_tree_to_tree(
          old_tree.map(|x| x.inner.deref()),
          new_tree.map(|x| x.inner.deref()),
          Some(&mut opts),
        )
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Diff { inner })
  }

  #[napi]
  /// Create a diff between two index objects.
  ///
  /// The first index will be used for the "oldFile" side of the delta, and
  /// the second index will be used for the "newFile" side of the delta.
  pub fn diff_index_to_index(
    &self,
    env: Env,
    this: Reference<Repository>,
    old_index: &Index,
    new_index: &Index,
    options: Option<DiffOptions>,
  ) -> crate::Result<Diff> {
    let mut opts: git2::DiffOptions = options.map(|x| x.into()).unwrap_or_default();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .diff_index_to_index(&old_index.inner, &new_index.inner, Some(&mut opts))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Diff { inner })
  }

  #[napi]
  /// Create a diff between the repository index and the workdir directory.
  ///
  /// This matches the `git diff` command.  See the note below on
  /// `diffTreeToWorkdir` for a discussion of the difference between
  /// `git diff` and `git diff HEAD` and how to emulate a `git diff <treeish>`
  /// using libgit2.
  ///
  /// The index will be used for the "oldFile" side of the delta, and the
  /// working directory will be used for the "newFile" side of the delta.
  ///
  /// If you pass `null` for the index, then the existing index of the `repo`
  /// will be used. In this case, the index will be refreshed from disk
  /// (if it has changed) before the diff is generated.
  pub fn diff_index_to_workdir(
    &self,
    env: Env,
    this: Reference<Repository>,
    index: Option<&Index>,
    options: Option<DiffOptions>,
  ) -> crate::Result<Diff> {
    let mut opts: git2::DiffOptions = options.map(|x| x.into()).unwrap_or_default();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .diff_index_to_workdir(index.map(|x| &x.inner), Some(&mut opts))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Diff { inner })
  }

  #[napi]
  /// Create a diff between a tree and the working directory.
  ///
  /// The tree you provide will be used for the "oldFile" side of the delta,
  /// and the working directory will be used for the "newFile" side.
  ///
  /// This is not the same as `git diff <treeish>` or `git diff-index <treeish>`.
  /// Those commands use information from the index, whereas this
  /// function strictly returns the differences between the tree and the files
  /// in the working directory, regardless of the state of the index. Use
  /// `diffTreeToWorkdirWithIndex` to emulate those commands.
  ///
  /// To see difference between this and `diffTreeToWorkdirWithIndex`,
  /// consider the example of a staged file deletion where the file has then
  /// been put back into the working dir and further modified. The
  /// tree-to-workdir diff for that file is 'modified', but `git diff` would
  /// show status 'deleted' since there is a staged delete.
  ///
  /// If `null` is passed for `tree`, then an empty tree is used.
  pub fn diff_tree_to_workdir(
    &self,
    this: Reference<Repository>,
    env: Env,
    old_tree: Option<&Tree>,
    options: Option<DiffOptions>,
  ) -> crate::Result<Diff> {
    let mut opts: git2::DiffOptions = options.map(|x| x.into()).unwrap_or_default();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .diff_tree_to_workdir(old_tree.map(|x| x.inner.deref()), Some(&mut opts))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Diff { inner })
  }

  #[napi]
  /// Create a diff between a tree and the working directory using index data
  /// to account for staged deletes, tracked files, etc.
  ///
  /// This emulates `git diff <tree>` by diffing the tree to the index and
  /// the index to the working directory and blending the results into a
  /// single diff that includes staged deleted, etc.
  pub fn diff_tree_to_workdir_with_index(
    &self,
    this: Reference<Repository>,
    env: Env,
    old_tree: Option<&Tree>,
    options: Option<DiffOptions>,
  ) -> crate::Result<Diff> {
    let mut opts: git2::DiffOptions = options.map(|x| x.into()).unwrap_or_default();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .diff_tree_to_workdir_with_index(old_tree.map(|x| x.inner.deref()), Some(&mut opts))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Diff { inner })
  }
}
