use crate::repository::Repository;
use napi_derive::napi;

#[napi(string_enum)]
pub enum FileFavor {
  /// When a region of a file is changed in both branches, a conflict will be
  /// recorded in the index so that git_checkout can produce a merge file with
  /// conflict markers in the working directory. This is the default.
  Normal,
  /// When a region of a file is changed in both branches, the file created
  /// in the index will contain the "ours" side of any conflicting region.
  /// The index will not record a conflict.
  Ours,
  /// When a region of a file is changed in both branches, the file created
  /// in the index will contain the "theirs" side of any conflicting region.
  /// The index will not record a conflict.
  Theirs,
  /// When a region of a file is changed in both branches, the file created
  /// in the index will contain each unique line from each side, which has
  /// the result of combining both files. The index will not record a conflict.
  Union,
}

impl From<FileFavor> for git2::FileFavor {
  fn from(value: FileFavor) -> git2::FileFavor {
    match value {
      FileFavor::Normal => git2::FileFavor::Normal,
      FileFavor::Ours => git2::FileFavor::Ours,
      FileFavor::Theirs => git2::FileFavor::Theirs,
      FileFavor::Union => git2::FileFavor::Union,
    }
  }
}

#[napi(object)]
pub struct MergeOptions {
  /// Detect file renames
  pub find_renames: Option<bool>,
  /// If a conflict occurs, exit immediately instead of attempting to continue
  /// resolving conflicts
  pub fail_on_conflict: Option<bool>,
  /// Do not write the REUC extension on the generated index
  pub skip_reuc: Option<bool>,
  /// If the commits being merged have multiple merge bases, do not build a
  /// recursive merge base (by merging the multiple merge bases), instead
  /// simply use the first base.
  pub no_recursive: Option<bool>,
  /// Similarity to consider a file renamed (default 50)
  pub rename_threshold: Option<u32>,
  ///  Maximum similarity sources to examine for renames (default 200).
  /// If the number of rename candidates (add / delete pairs) is greater
  /// than this value, inexact rename detection is aborted. This setting
  /// overrides the `merge.renameLimit` configuration value.
  pub target_limit: Option<u32>,
  /// Maximum number of times to merge common ancestors to build a
  /// virtual merge base when faced with criss-cross merges.  When
  /// this limit is reached, the next ancestor will simply be used
  /// instead of attempting to merge it.  The default is unlimited.
  pub recursion_limit: Option<u32>,
  /// Specify a side to favor for resolving conflicts
  pub fil_favor: Option<FileFavor>,
  /// Create standard conflicted merge files
  pub standard_style: Option<bool>,
  /// Create diff3-style file
  pub diff3_style: Option<bool>,
  /// Condense non-alphanumeric regions for simplified diff file
  pub simplify_alnum: Option<bool>,
  /// Ignore all whitespace
  pub ignore_whitespace: Option<bool>,
  /// Ignore changes in amount of whitespace
  pub ignore_whitespace_change: Option<bool>,
  /// Ignore whitespace at end of line
  pub ignore_whitespace_eol: Option<bool>,
  /// Use the "patience diff" algorithm
  pub patience: Option<bool>,
  /// Take extra time to find minimal diff
  pub minimal: Option<bool>,
}

impl From<MergeOptions> for git2::MergeOptions {
  fn from(value: MergeOptions) -> Self {
    let mut options = git2::MergeOptions::new();
    if let Some(find) = value.find_renames {
      options.find_renames(find);
    }
    if let Some(fail) = value.fail_on_conflict {
      options.fail_on_conflict(fail);
    }
    if let Some(skip) = value.skip_reuc {
      options.skip_reuc(skip);
    }
    if let Some(disable) = value.no_recursive {
      options.no_recursive(disable);
    }
    if let Some(thresh) = value.rename_threshold {
      options.rename_threshold(thresh);
    }
    if let Some(limit) = value.target_limit {
      options.target_limit(limit);
    }
    if let Some(limit) = value.recursion_limit {
      options.recursion_limit(limit);
    }
    if let Some(favor) = value.fil_favor {
      options.file_favor(git2::FileFavor::from(favor));
    }
    if let Some(standard) = value.standard_style {
      options.standard_style(standard);
    }
    if let Some(diff3) = value.diff3_style {
      options.diff3_style(diff3);
    }
    if let Some(simplify) = value.simplify_alnum {
      options.simplify_alnum(simplify);
    }
    if let Some(ignore) = value.ignore_whitespace {
      options.ignore_whitespace(ignore);
    }
    if let Some(ignore) = value.ignore_whitespace_change {
      options.ignore_whitespace_change(ignore);
    }
    if let Some(ignore) = value.ignore_whitespace_eol {
      options.ignore_whitespace_eol(ignore);
    }
    if let Some(patience) = value.patience {
      options.patience(patience);
    }
    if let Some(minimal) = value.minimal {
      options.minimal(minimal);
    }
    options
  }
}

#[napi]
impl Repository {
  #[napi]
  pub fn merge(&self) -> crate::Result<()> {
    todo!()
  }
}
