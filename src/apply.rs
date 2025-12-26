use crate::diff::Diff;
use crate::index::Index;
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

#[napi(object, object_to_js = false)]
/// Options to specify when applying a diff
pub struct ApplyOptions {
  /// Don't actually make changes, just test that the patch applies.
  pub check: Option<bool>,
  // TODO(@seokju-na): Consider node.js is single-thread so the calling callback from Rust side
  // will make dead-lock. May be we should make `apply` function as async?
  //
  // #[napi(ts_type = "(data: DiffHunkData | null | undefined) => boolean")]
  // pub hunk_callback: Option<HunkCallback>,
  // #[napi(ts_type = "(data: DeltaData | null | undefined) => boolean")]
  // pub delta_callback: Option<DeltaCallback>,
}

impl From<ApplyOptions> for git2::ApplyOptions<'_> {
  fn from(value: ApplyOptions) -> Self {
    let mut options = git2::ApplyOptions::new();
    if let Some(check) = value.check {
      options.check(check);
    }
    options
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Apply a Diff to the given repo, making changes directly in the working directory, the index, or both.
  ///
  /// @category Repository/Methods
  /// ```ts
  /// class Repository {
  ///   apply(diff: Diff, location: ApplyLocation, options?: ApplyOptions | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {Diff} diff - The diff to apply
  /// @param {ApplyLocation} location - The location to apply
  /// @param {ApplyOptions} [options] - The options for the apply
  pub fn apply(&self, diff: &Diff, location: ApplyLocation, options: Option<ApplyOptions>) -> crate::Result<()> {
    self
      .inner
      .apply(&diff.inner, location.into(), options.map(|x| x.into()).as_mut())?;
    Ok(())
  }

  #[napi]
  /// Apply a Diff to the provided tree, and return the resulting Index.
  ///
  /// @category Repository/Methods
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
  /// @param {Tree} tree - The tree to apply the diff to
  /// @param {Diff} diff - The diff to apply
  /// @param {ApplyOptions} [options] - The options for the apply
  ///
  /// @returns The postimage of the application
  pub fn apply_to_tree(&self, tree: &Tree, diff: &Diff, options: Option<ApplyOptions>) -> crate::Result<Index> {
    let inner = self
      .inner
      .apply_to_tree(&tree.inner, &diff.inner, options.map(|x| x.into()).as_mut())?;
    Ok(Index { inner })
  }
}
