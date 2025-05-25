use crate::annotated_commit::AnnotatedCommit;
use crate::checkout::CheckoutOptions;
use crate::commit::Commit;
use crate::index::Index;
use crate::reference::Reference;
use crate::repository::Repository;
use crate::tree::Tree;
use napi_derive::napi;
use std::ops::Deref;

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

#[napi(object)]
pub struct MergeAnalysis {
  /// No merge is possible.
  pub none: bool,
  /// A "normal" merge; both HEAD and the given merge input have diverged
  /// from their common ancestor. The divergent commits must be merged.
  pub normal: bool,
  /// All given merge inputs are reachable from HEAD, meaning the
  /// repository is up-to-date and no merge needs to be performed.
  pub up_to_date: bool,
  /// The given merge input is a fast-forward from HEAD and no merge
  /// needs to be performed.  Instead, the client can check out the
  /// given merge input.
  pub fast_forward: bool,
  /// The HEAD of the current repository is "unborn" and does not point to
  /// a valid commit.  No merge can be performed, but the caller may wish
  /// to simply set HEAD to the target commit(s).
  pub unborn: bool,
}

impl From<git2::MergeAnalysis> for MergeAnalysis {
  fn from(value: git2::MergeAnalysis) -> Self {
    let none = value.is_none();
    let normal = value.is_normal();
    let up_to_date = value.is_up_to_date();
    let fast_forward = value.is_fast_forward();
    let unborn = value.is_unborn();
    Self {
      none,
      normal,
      up_to_date,
      fast_forward,
      unborn,
    }
  }
}

#[napi(object)]
pub struct MergePreference {
  /// No configuration was found that suggests a preferred behavior for
  /// merge.
  pub none: bool,
  /// There is a `merge.ff=false` configuration setting, suggesting that
  /// the user does not want to allow a fast-forward merge.
  pub no_fast_forward: bool,
  /// There is a `merge.ff=only` configuration setting, suggesting that
  /// the user only wants fast-forward merges.
  pub fast_forward_only: bool,
}

impl From<git2::MergePreference> for MergePreference {
  fn from(value: git2::MergePreference) -> Self {
    let none = value.is_none();
    let no_fast_forward = value.is_no_fast_forward();
    let fast_forward_only = value.is_fastforward_only();
    Self {
      none,
      no_fast_forward,
      fast_forward_only,
    }
  }
}

#[napi(object)]
pub struct MergeAnalysisResult {
  pub analysis: MergeAnalysis,
  pub preference: MergePreference,
}

#[napi]
impl Repository {
  #[napi]
  /// Merges the given commit(s) into HEAD, writing the results into the
  /// working directory. Any changes are staged for commit and any conflicts
  /// are written to the index. Callers should inspect the repository's index
  /// after this completes, resolve any conflicts and prepare a commit.
  ///
  /// For compatibility with git, the repository is put into a merging state.
  /// Once the commit is done (or if the user wishes to abort), you should
  /// clear this state by calling `cleanupState()`.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   merge(
  ///     annotatedCommits: AnnotatedCommit[],
  ///     mergeOptions?: MergeOptions | undefined | null,
  ///     checkoutOptions?: CheckoutOptions | undefined | null,
  ///   ): void;
  /// }
  /// ```
  ///
  /// @param {AnnotatedCommit[]} annotatedCommits -
  /// @param {MergeOptions} [mergeOptions] -
  /// @param {CheckoutOptions} [checkoutOptions] -
  pub fn merge(
    &self,
    annotated_commits: Vec<&AnnotatedCommit>,
    merge_options: Option<MergeOptions>,
    checkout_options: Option<CheckoutOptions>,
  ) -> crate::Result<()> {
    let commits = annotated_commits.iter().map(|x| x.inner.deref()).collect::<Vec<_>>();
    let mut merge_opts = merge_options.map(git2::MergeOptions::from);
    let mut checkout_opts = checkout_options.map(git2::build::CheckoutBuilder::from);
    self
      .inner
      .merge(commits.as_slice(), merge_opts.as_mut(), checkout_opts.as_mut())?;
    Ok(())
  }

  #[napi]
  /// Merge two commits, producing an index that reflects the result of
  /// the merge. The index may be written as-is to the working directory or
  /// checked out. If the index is to be converted to a tree, the caller
  /// should resolve any conflicts that arose as part of the merge.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   mergeCommits(
  ///     ourCommit: Commit,
  ///     theirCommit: Commit,
  ///     options?: MergeOptions | undefined | null,
  ///   ): Index;
  /// }
  /// ```
  ///
  /// @param {Commit} outCommit -
  /// @param {Commit} theirCommit -
  /// @param {MergeOptions} [options] -
  /// @returns
  pub fn merge_commits(
    &self,
    our_commit: &Commit,
    their_commit: &Commit,
    options: Option<MergeOptions>,
  ) -> crate::Result<Index> {
    let opts = options.map(git2::MergeOptions::from);
    let inner = self
      .inner
      .merge_commits(&our_commit.inner, &their_commit.inner, opts.as_ref())?;
    Ok(Index { inner })
  }

  #[napi]
  /// Merge two trees, producing an index that reflects the result of
  /// the merge. The index may be written as-is to the working directory or
  /// checked out. If the index is to be converted to a tree, the caller
  /// should resolve any conflicts that arose as part of the merge.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   mergeTrees(
  ///     ancestorTree: Tree,
  ///     ourTree: Tree,
  ///     theirTree: Tree,
  ///     options?: MergeOptions | undefined | null,
  ///   ): Index;
  /// }
  /// ```
  ///
  /// @param {Tree} ancestorTree -
  /// @param {Tree} outTree -
  /// @param {Tree} theirTree -
  /// @param {MergeOptions} [options] -
  /// @returns
  pub fn merge_trees(
    &self,
    ancestor_tree: &Tree,
    our_tree: &Tree,
    their_tree: &Tree,
    options: Option<MergeOptions>,
  ) -> crate::Result<Index> {
    let opts = options.map(git2::MergeOptions::from);
    let inner = self
      .inner
      .merge_trees(&ancestor_tree.inner, &our_tree.inner, &their_tree.inner, opts.as_ref())?;
    Ok(Index { inner })
  }

  #[napi]
  /// Analyzes the given branch(es) and determines the opportunities for
  /// merging them into the HEAD of the repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   mergeAnalysis(theirHeads: AnnotatedCommit[]): MergeAnalysisResult;
  /// }
  /// ```
  ///
  /// @param {AnnotatedCommit[]} theirHeads -
  /// @returns
  pub fn merge_analysis(&self, their_heads: Vec<&AnnotatedCommit>) -> crate::Result<MergeAnalysisResult> {
    let commits = their_heads.iter().map(|x| x.inner.deref()).collect::<Vec<_>>();
    let (analysis, preference) = self.inner.merge_analysis(commits.as_slice())?;
    Ok(MergeAnalysisResult {
      analysis: analysis.into(),
      preference: preference.into(),
    })
  }

  #[napi]
  /// Analyzes the given branch(es) and determines the opportunities for
  /// merging them into a reference.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   mergeAnalysisForRef(ourRef: Reference, theirHeads: AnnotatedCommit[]): MergeAnalysisResult;
  /// }
  /// ```
  ///
  /// @param {Reference} ourRef -
  /// @param {AnnotatedCommit[]} theirHeads -
  /// @returns
  pub fn merge_analysis_for_ref(
    &self,
    our_ref: &Reference,
    their_heads: Vec<&AnnotatedCommit>,
  ) -> crate::Result<MergeAnalysisResult> {
    let commits = their_heads.iter().map(|x| x.inner.deref()).collect::<Vec<_>>();
    let (analysis, preference) = self.inner.merge_analysis_for_ref(&our_ref.inner, commits.as_slice())?;
    Ok(MergeAnalysisResult {
      analysis: analysis.into(),
      preference: preference.into(),
    })
  }
}
