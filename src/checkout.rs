use crate::index::Index;
use crate::object::GitObject;
use crate::repository::Repository;
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
pub struct CheckoutOptions {
  pub dry_run: Option<bool>,
  pub force: Option<bool>,
  pub safe: Option<bool>,
  pub recreate_missing: Option<bool>,
  pub allow_conflicts: Option<bool>,
  pub remove_untracked: Option<bool>,
  pub remove_ignored: Option<bool>,
  pub update_only: Option<bool>,
  pub update_index: Option<bool>,
  pub refresh: Option<bool>,
  pub skip_unmerged: Option<bool>,
  pub use_ours: Option<bool>,
  pub use_theirs: Option<bool>,
  pub overwrite_ignored: Option<bool>,
  pub conflict_style_merge: Option<bool>,
  pub conflict_style_diff3: Option<bool>,
  pub disable_pathspec_match: Option<bool>,
  pub disable_filters: Option<bool>,
  pub dir_perm: Option<i32>,
  pub file_perm: Option<i32>,
  pub path: Option<String>,
  pub target_dir: Option<String>,
  pub ancestor_label: Option<String>,
  pub our_label: Option<String>,
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
  pub fn checkout_head(&self, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    self.inner.checkout_head(builder.as_mut())?;
    Ok(())
  }

  #[napi]
  pub fn checkout_index(&self, index: Option<&Index>, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    let mut git_index = index.map(|x| x.inner); // todo: impl copy
    self.inner.checkout_index(git_index.as_mut(), builder.as_mut())?;
    Ok(())
  }

  #[napi]
  pub fn checkout_tree(&self, treeish: &GitObject, options: Option<CheckoutOptions>) -> crate::Result<()> {
    let mut builder = options.map(git2::build::CheckoutBuilder::from);
    self.inner.checkout_tree(&treeish.inner, builder.as_mut())?;
    Ok(())
  }
}
