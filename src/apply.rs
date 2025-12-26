use crate::diff::Diff;
use crate::index::Index;
use crate::js::{JsCallback, JsCallbackExt};
use crate::repository::Repository;
use crate::tree::Tree;
use napi_derive::napi;

#[derive(Debug, Copy, Clone)]
#[napi(string_enum)]
/// Possible application locations for git_apply
/// see <https://libgit2.org/libgit2/#HEAD/type/git_apply_options>
pub enum ApplyLocation {
  /// Apply the patch to the workdir
  WorkDir,
  /// Apply the patch to the index
  Index,
  /// Apply the patch to both the working directory and the index
  Both,
}

impl From<ApplyLocation> for git2::ApplyLocation {
  fn from(value: ApplyLocation) -> Self {
    match value {
      ApplyLocation::Both => Self::Both,
      ApplyLocation::Index => Self::Index,
      ApplyLocation::WorkDir => Self::WorkDir,
    }
  }
}

#[napi(object)]
/// Structure describing a hunk of a diff.
pub struct DiffHunkData {
  /// Starting line number in old_file
  pub old_start: u32,
  /// Number of lines in old_file
  pub old_lines: u32,
  /// Starting line number in new_filenew_start: u32,
  pub new_start: u32,
  /// Number of lines in new_file
  pub new_lines: u32,
  /// Header text
  pub header: String,
}

impl TryFrom<git2::DiffHunk<'_>> for DiffHunkData {
  type Error = crate::Error;

  fn try_from(value: git2::DiffHunk<'_>) -> std::result::Result<Self, Self::Error> {
    let old_start = value.old_start();
    let old_lines = value.old_lines();
    let new_start = value.new_start();
    let new_lines = value.new_lines();
    let header = std::str::from_utf8(value.header())?;
    Ok(Self {
      old_start,
      old_lines,
      new_start,
      new_lines,
      header: header.to_string(),
    })
  }
}

type HunkCallback = JsCallback<Option<DiffHunkData>, bool>;

#[napi(object, object_to_js = false)]
/// Options to specify when applying a diff
pub struct ApplyOptions {
  /// Don't actually make changes, just test that the patch applies.
  pub check: Option<bool>,
  /// When applying a patch, callback that will be made per hunk.
  #[napi(ts_type = "(data: DiffHunkData | null | undefined) => boolen")]
  pub hunk_callback: Option<HunkCallback>,
  // TODO: How can we pass DiffDelta as a callback parameter?
  // pub delta_callback: Option<DeltaCallback>,
}

impl<'a> From<ApplyOptions> for git2::ApplyOptions<'a> {
  fn from(value: ApplyOptions) -> Self {
    let mut options = git2::ApplyOptions::new();
    if let Some(check) = value.check {
      options.check(check);
    }
    if let Some(callback) = value.hunk_callback {
      options.hunk_callback(move |hunk| {
        let data = hunk.and_then(|x| DiffHunkData::try_from(x).ok());
        callback.invoke(data).unwrap_or(true)
      });
    }
    options
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Apply a Diff to the given repo, making changes directly in the working directory, the index, or both.
  ///
  /// @category Repositoryodssignature
  /// ```ts
  /// class Repository {
  ///   apply(diff: Diff, location: ApplyLocation, options?: ApplyOptions | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {Diff} diff -
  /// @param {ApplyLocation} location -
  /// @param {ApplyOptions} [options] -
  pub fn apply(&self, diff: &Diff, location: ApplyLocation, options: Option<ApplyOptions>) -> crate::Result<()> {
    self
      .inner
      .apply(&diff.inner, location.into(), options.map(|x| x.into()).as_mut())?;
    Ok(())
  }

  #[napi]
  /// Apply a Diff to the provided tree, and return the resulting Index.
  ///
  /// @category Repositoryods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   applyToTree(
  ///     tree: Tree,
  ///     diff: Diff,
  ///     options?: ApplyOptions | null | undefined
  ///   ): Index;
  /// }
  /// ```
  ///
  /// @param {Tree} tree -
  /// @param {Diff} diff -
  /// @param {ApplyOptions} [options] -
  ///
  /// @returns
  pub fn apply_to_tree(&self, tree: &Tree, diff: &Diff, options: Option<ApplyOptions>) -> crate::Result<Index> {
    let inner = self
      .inner
      .apply_to_tree(&tree.inner, &diff.inner, options.map(|x| x.into()).as_mut())?;
    Ok(Index { inner })
  }
}
