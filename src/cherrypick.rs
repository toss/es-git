use crate::checkout::CheckoutOptions;
use crate::commit::Commit;
use crate::index::Index;
use crate::merge::MergeOptions;
use crate::repository::Repository;
use napi_derive::napi;

/// Options for cherrypick behavior.
#[napi(object)]
pub struct CherrypickOptions {
  /// Parent number for merge commits (1-based).
  ///
  /// When cherrypicking a merge commit, the mainline parent is the one you want to
  /// cherrypick from. The mainline is the branch from which the merge was made.
  pub mainline: Option<u32>,
  /// Options for merge resolution when cherrypicking a merge commit.
  pub merge_options: Option<MergeOptions>,
  /// Options for checkout behavior when updating working directory.
  pub checkout_options: Option<CheckoutOptions>,
}

//Convert napi object to git2::CherrypickOptions
impl From<CherrypickOptions> for git2::CherrypickOptions<'static> {
  fn from(value: CherrypickOptions) -> Self {
    let mut git2_cherrypick_options = git2::CherrypickOptions::new();
    if let Some(mainline) = value.mainline {
      git2_cherrypick_options.mainline(mainline);
    }
    if let Some(merge_opts) = value.merge_options {
      git2_cherrypick_options.merge_opts(merge_opts.into());
    }
    if let Some(checkout_opts) = value.checkout_options {
      git2_cherrypick_options.checkout_builder(checkout_opts.into());
    }
    git2_cherrypick_options
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Cherrypicks the given commit onto HEAD and updates the working tree and index.
  /// This method prepares the index and tree as if the commit were applied, but does not actually make a new commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   cherrypick(
  ///     commit: Commit,
  ///     options?: CherrypickOptions | undefined | null,
  ///   ): void;
  /// }
  /// ```
  ///
  /// @param {Commit} commit - The commit to cherrypick.
  /// @param {CherrypickOptions} [options] - Options for the cherrypick operation.
  /// @throws {Error} If the commit is a merge commit and no mainline is specified.
  /// @throws {Error} If there are conflicts during the cherrypick operation.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./path/to/repo');
  /// const cherrypickCommit = repo.getCommit('cherrypick-commit');
  ///
  /// // Cherrypick the commit onto HEAD and working tree
  /// repo.cherrypick(cherrypickCommit);
  /// repo.cleanupState();
  ///
  /// // Cherrypick the commit against our commit selecting the first parent as mainline (This is necessary because, for merge commits, there is ambiguity about which side of the merge should be treated as the baseline.)
  /// repo.cherrypick(cherrypickCommit, { mainline: 1 });
  /// repo.cleanupState();
  ///
  /// // Prevent working tree changes (dry run) but compute conflicts
  /// repo.cherrypick(cherrypickCommit, { checkoutOptions: { dryRun: true } });
  /// repo.cleanupState();
  ///
  /// // Cherrypick the commit against our commit selecting the first parent as mainline and prevent working tree changes (dry run) but compute conflicts
  /// repo.cherrypick(cherrypickCommit, { mainline: 1, checkoutOptions: { dryRun: true } });
  /// repo.cleanupState();
  /// ```
  pub fn cherrypick(&self, commit: &Commit, options: Option<CherrypickOptions>) -> crate::Result<()> {
    let mut git2_cherrypick_options = options.map(Into::into);
    self
      .inner
      .cherrypick(&commit.inner, git2_cherrypick_options.as_mut())
      .map_err(crate::Error::from)?;
    Ok(())
  }

  #[napi]
  /// Applies a cherrypick of `cherrypickCommit` against `ourCommit` and returns the resulting Index,
  /// without modifying the working directory or repository state.  
  /// This method does not write any changes to disk or update HEAD.
  /// it is useful for computing what the cherrypick result would look like without actually applying it.  
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   cherrypickCommit(
  ///     cherrypickCommit: Commit,
  ///     ourCommit: Commit,
  ///     mainline: number,
  ///     mergeOptions?: MergeOptions | undefined | null,
  ///   ): Index;
  /// }
  /// ```
  ///
  /// @param {Commit} cherrypickCommit - The commit to cherrypick.
  /// @param {Commit} ourCommit - The commit to cherrypick against (usually HEAD).
  /// @param {number} mainline - The parent of the cherrypick commit, if it is a merge (1-based).
  /// @param {MergeOptions} [mergeOptions] - Options for merge conflict resolution.
  /// @returns The index result.
  /// @throws {Error} If the cherrypick commit is a merge and mainline is 0.
  /// @throws {Error} If there are conflicts and failOnConflict is true (default).
  ///
  /// @example
  /// ```ts
  /// // This is an example for cherrypick_commit
  /// import { openRepository } from "es-git";
  ///
  /// const repo = await openRepository("./path/to/repo");
  /// const cherry = repo.getCommit("cherrypick-commit");
  /// const target = repo.getCommit("onto-commit");
  ///
  /// // Returns the Index resulting from the cherrypick in memory,
  /// // without affecting HEAD or the working tree.
  /// // The mainline parameter indicates which parent to use as the baseline,
  /// // For merge commits, mainline specifies which parent to use as baseline (1 or 2).
  /// // For normal (non-merge) commits, use mainline 0.
  /// const idx = repo.cherrypickCommit(cherry, target, 0);
  ///
  /// // You can check for conflicts with idx.hasConflicts()
  /// ```
  pub fn cherrypick_commit(
    &self,
    cherrypick_commit: &Commit,
    our_commit: &Commit,
    mainline: u32,
    merge_options: Option<MergeOptions>,
  ) -> crate::Result<Index> {
    let git2_merge_options = merge_options.map(Into::into);
    let git2_index = self
      .inner
      .cherrypick_commit(
        &cherrypick_commit.inner,
        &our_commit.inner,
        mainline,
        git2_merge_options.as_ref(),
      )
      .map_err(crate::Error::from)?;
    Ok(Index { inner: git2_index })
  }
}
