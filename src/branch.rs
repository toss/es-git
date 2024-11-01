use git2::BranchType as Git2BranchType;
use git2::Repository;

use crate::context::GitContext;

#[napi(object)]
pub struct Branch {
  pub name: String,
  pub oid: String,
}

#[napi(object)]
pub struct CreateBranchOptions {
  pub branch_name: String,
  pub target_sha: String,
  pub force: Option<bool>,
}

#[napi(object)]
pub struct GetBranchOptions {
  pub branch_name: String,
}

#[napi(object)]
pub struct DeleteBranchOptions {
  pub branch_name: String,
}

#[napi]
pub fn create_branch(options: CreateBranchOptions, context: GitContext) -> anyhow::Result<Branch> {
  let repo = Repository::open(&context.dir)?;

  let force = options.force.unwrap_or(false);
  let object = repo.revparse_single(&options.target_sha)?;
  let commit = object.peel_to_commit()?;

  let branch = repo.branch(&options.branch_name, &commit, force)?;
  let name = branch
    .name()?
    .map(|name| name.to_string())
    .expect("Branch name should be a valid UTF-8 string");
  let oid = branch.get().peel_to_commit()?.id().to_string();

  Ok(Branch { name, oid })
}

#[napi]
pub fn get_branch(options: GetBranchOptions, context: GitContext) -> anyhow::Result<Branch> {
  let repo = Repository::open(&context.dir)?;

  let branch = repo.find_branch(&options.branch_name, Git2BranchType::Local)?;

  let name = branch
    .name()?
    .map(|name| name.to_string())
    .expect("Branch name should be a valid UTF-8 string");

  let oid = branch.get().peel_to_commit()?.id().to_string();

  Ok(Branch { name, oid })
}

#[napi]
pub fn delete_branch(options: DeleteBranchOptions, context: GitContext) -> anyhow::Result<()> {
  let repo = Repository::open(&context.dir)?;
  let mut branch = repo.find_branch(&options.branch_name, Git2BranchType::Local)?;

  branch.delete().ok();

  Ok(())
}
