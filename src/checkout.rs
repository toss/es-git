use crate::index::Index;
use crate::object::GitObject;
use crate::repository::Repository;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
pub struct CheckoutOptions {
  /// Indicate that this checkout should perform a dry run by checking for
  /// conflicts but not make any actual changes.
  pub dry_run: Option<bool>,
  /// Take any action necessary to get the working directory to match the
  /// target including potentially discarding modified files.
  pub force: Option<bool>,
  /// Indicate that the checkout should be performed safely, allowing new
  /// files to be created but not overwriting existing files or changes.
  ///
  /// This is the default.
  pub safe: Option<bool>,
  /// In safe mode, create files that don't exist.
  ///
  /// Defaults to false.
  pub recreate_missing: Option<bool>,
  /// In safe mode, apply safe file updates even when there are conflicts
  /// instead of canceling the checkout.
  ///
  /// Defaults to false.
  pub allow_conflicts: Option<bool>,
  /// Remove untracked files from the working dir.
  ///
  /// Defaults to false.
  pub remove_untracked: Option<bool>,
  /// Remove ignored files from the working dir.
  ///
  /// Defaults to false.
  pub remove_ignored: Option<bool>,
  /// Only update the contents of files that already exist.
  ///
  /// If set, files will not be created or deleted.
  ///
  /// Defaults to false.
  pub update_only: Option<bool>,
  /// Prevents checkout from writing the updated files' information to the
  /// index.
  ///
  /// Defaults to true.
  pub update_index: Option<bool>,
  /// Indicate whether the index and git attributes should be refreshed from
  /// disk before any operations.
  ///
  /// Defaults to true,
  pub refresh: Option<bool>,
  /// Skip files with unmerged index entries.
  ///
  /// Defaults to false.
  pub skip_unmerged: Option<bool>,
  /// Indicate whether the checkout should proceed on conflicts by using the
  /// stage 2 version of the file ("ours").
  ///
  /// Defaults to false.
  pub use_ours: Option<bool>,
  /// Indicate whether the checkout should proceed on conflicts by using the
  /// stage 3 version of the file ("theirs").
  ///
  /// Defaults to false.
  pub use_theirs: Option<bool>,
  /// Indicate whether ignored files should be overwritten during the checkout.
  ///
  /// Defaults to true.
  pub overwrite_ignored: Option<bool>,
  /// Indicate whether a normal merge file should be written for conflicts.
  ///
  /// Defaults to false.
  pub conflict_style_merge: Option<bool>,
  /// Indicates whether to include common ancestor data in diff3 format files
  /// for conflicts.
  ///
  /// Defaults to false.
  pub conflict_style_diff3: Option<bool>,
  /// Treat paths specified in `path` as exact file paths
  /// instead of as pathspecs.
  pub disable_pathspec_match: Option<bool>,
  /// Indicate whether to apply filters like CRLF conversion.
  pub disable_filters: Option<bool>,
  /// Set the mode with which new directories are created.
  ///
  /// Default is 0755
  pub dir_perm: Option<i32>,
  /// Set the mode with which new files are created.
  ///
  /// The default is 0644 or 0755 as dictated by the blob.
  pub file_perm: Option<i32>,
  /// Add a path to be checked out.
  ///
  /// The path is a [pathspec](https://git-scm.com/docs/gitglossary.html#Documentation/gitglossary.txt-aiddefpathspecapathspec) pattern, unless
  /// `disablePathspecMatch` is set.
  ///
  /// If no paths are specified, then all files are checked out. Otherwise
  /// only these specified paths are checked out.
  pub path: Option<String>,
  /// Set the directory to check out to
  pub target_dir: Option<String>,
  /// The name of the common ancestor side of conflicts
  pub ancestor_label: Option<String>,
  /// The name of the common our side of conflicts
  pub our_label: Option<String>,
  /// The name of the common their side of conflicts
  pub their_label: Option<String>,
}

impl From<CheckoutOptions> for git2::build::CheckoutBuilder<'static> {
  fn from(value: CheckoutOptions) -> Self {
    let mut builder = git2::build::CheckoutBuilder::new();
    if let Some(true) = value.dry_run {
      builder.dry_run();
    }
    if let Some(true) = value.force {
      builder.force();
    }
    if let Some(true) = value.safe {
      builder.safe();
    }
    if let Some(allow) = value.recreate_missing {
      builder.recreate_missing(allow);
    }
    if let Some(allow) = value.allow_conflicts {
      builder.allow_conflicts(allow);
    }
    if let Some(remove) = value.remove_untracked {
      builder.remove_untracked(remove);
    }
    if let Some(remove) = value.remove_ignored {
      builder.remove_ignored(remove);
    }
    if let Some(update) = value.update_only {
      builder.update_only(update);
    }
    if let Some(update) = value.update_index {
      builder.update_index(update);
    }
    if let Some(refresh) = value.refresh {
      builder.refresh(refresh);
    }
    if let Some(skip) = value.skip_unmerged {
      builder.skip_unmerged(skip);
    }
    if let Some(ours) = value.use_ours {
      builder.use_ours(ours);
    }
    if let Some(theirs) = value.use_theirs {
      builder.use_theirs(theirs);
    }
    if let Some(overwrite) = value.overwrite_ignored {
      builder.overwrite_ignored(overwrite);
    }
    if let Some(on) = value.conflict_style_merge {
      builder.conflict_style_merge(on);
    }
    if let Some(on) = value.conflict_style_diff3 {
      builder.conflict_style_diff3(on);
    }
    if let Some(on) = value.disable_pathspec_match {
      builder.disable_pathspec_match(on);
    }
    if let Some(disable) = value.disable_filters {
      builder.disable_filters(disable);
    }
    if let Some(perm) = value.dir_perm {
      builder.dir_perm(perm);
    }
    if let Some(perm) = value.file_perm {
      builder.file_perm(perm);
    }
    if let Some(path) = value.path {
      builder.path(path);
    }
    if let Some(dst) = value.target_dir {
      builder.target_dir(Path::new(&dst));
    }
    if let Some(label) = value.ancestor_label {
      builder.ancestor_label(&label);
    }
    if let Some(label) = value.our_label {
      builder.our_label(&label);
    }
    if let Some(label) = value.their_label {
      builder.their_label(&label);
    }
    builder
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Updates files in the index and the working tree to match the content of
  /// the commit pointed at by HEAD.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   checkoutHead(options?: CheckoutOptions | undefined | null): void;
  /// }
  /// ```
  ///
  /// @param {CheckoutOptions} [options] - Options for checkout.
  pub fn checkout_head(&self, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    self.inner.checkout_head(builder.as_mut())?;
    Ok(())
  }

  #[napi]
  /// Updates files in the working tree to match the content of the index.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   checkoutIndex(
  ///     index?: Index | undefined | null,
  ///     options?: CheckoutOptions | undefined | null,
  ///   ): void;
  /// }
  /// ```
  ///
  /// @param {Index} [index] - Index to checkout. If not given, the repository's index will be used.
  /// @param {CheckoutOptions} [options] - Options for checkout.
  pub fn checkout_index(&self, index: Option<&mut Index>, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    let git_index = index.map(|x| &mut x.inner);
    self.inner.checkout_index(git_index, builder.as_mut())?;
    Ok(())
  }

  #[napi]
  /// Updates files in the index and working tree to match the content of the
  /// tree pointed at by the treeish.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   checkoutTree(treeish: GitObject, options?: CheckoutOptions | undefined | null): void;
  /// }
  /// ```
  ///
  /// @param {GitObject} treeish - Git object which tree pointed.
  /// @param {CheckoutOptions} [options] - Options for checkout.
  pub fn checkout_tree(&self, treeish: &GitObject, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    self.inner.checkout_tree(&treeish.inner, builder.as_mut())?;
    Ok(())
  }
}
