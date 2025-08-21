use crate::checkout::CheckoutOptions;
use crate::commit::Commit;
use crate::index::Index;
use crate::merge::MergeOptions;
use crate::repository::Repository;
use napi_derive::napi;

#[napi(object)]
pub struct RevertOptions {
  /// Parent number for merge commits (1-based).
  ///
  /// When reverting a merge commit, the mainline parent is the one you want to
  /// revert to. The mainline is the branch into which the merge was made.
  pub mainline: Option<u32>,
  /// Options for merge conflict resolution.
  pub merge_options: Option<MergeOptions>,
  /// Options for checkout behavior when updating working directory.
  pub checkout_options: Option<CheckoutOptions>,
}

impl From<RevertOptions> for git2::RevertOptions<'static> {
  fn from(value: RevertOptions) -> Self {
    let mut opts = git2::RevertOptions::new();

    if let Some(mainline) = value.mainline {
      opts.mainline(mainline);
    }

    if let Some(merge_opts) = value.merge_options {
      opts.merge_opts(merge_opts.into());
    }

    if let Some(checkout_opts) = value.checkout_options {
      opts.checkout_builder(checkout_opts.into());
    }

    opts
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Reverts the given commit, applying the inverse of its changes to the
  /// HEAD commit and the working directory.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   revert(
  ///     commit: Commit,
  ///     options?: RevertOptions | undefined | null,
  ///   ): void;
  /// }
  /// ```
  ///
  /// @param {Commit} commit - The commit to revert.
  /// @param {RevertOptions} [options] - Options for the revert operation.
  /// @throws {Error} If the commit is a merge commit and no mainline is specified.
  /// @throws {Error} If there are conflicts during the revert operation.
  pub fn revert(&self, commit: &Commit, options: Option<RevertOptions>) -> crate::Result<()> {
    let mut git_options = options.map(Into::into);

    self
      .inner
      .revert(&commit.inner, git_options.as_mut())
      .map_err(crate::Error::from)?;

    Ok(())
  }

  #[napi]
  /// Reverts the given commit against the given "our" commit, producing an
  /// index that reflects the result of the revert.
  ///
  /// The returned index must be written to disk for the changes to take effect.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   revertCommit(
  ///     revertCommit: Commit,
  ///     ourCommit: Commit,
  ///     mainline: number,
  ///     mergeOptions?: MergeOptions | undefined | null,
  ///   ): Index;
  /// }
  /// ```
  ///
  /// @param {Commit} revertCommit - The commit to revert.
  /// @param {Commit} ourCommit - The commit to revert against (usually HEAD).
  /// @param {number} mainline - The parent of the revert commit, if it is a merge (1-based).
  /// @param {MergeOptions} [mergeOptions] - Options for merge conflict resolution.
  /// @returns The index result.
  pub fn revert_commit(
    &self,
    revert_commit: &Commit,
    our_commit: &Commit,
    mainline: u32,
    merge_options: Option<MergeOptions>,
  ) -> crate::Result<Index> {
    let opts = merge_options.map(git2::MergeOptions::from);
    let inner = self
      .inner
      .revert_commit(&revert_commit.inner, &our_commit.inner, mainline, opts.as_ref())?;
    Ok(Index { inner })
  }
}
