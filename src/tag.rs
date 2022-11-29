use git2::Repository;

use crate::context::GitContext;

#[napi(object)]
pub struct CreateTagOptions {
  pub name: String,
  pub message: String,
  pub sha: String,
}

#[napi(object)]
pub struct CreateTagResult {
  pub oid: String,
}

#[napi]
pub fn create_tag(
  options: CreateTagOptions,
  context: GitContext,
) -> anyhow::Result<CreateTagResult> {
  let repo = Repository::open(context.dir)?;
  let sig = repo.signature()?;
  let obj = repo.revparse_single(&options.sha)?;

  let oid = repo.tag(&options.name, &obj, &sig, &options.message, false)?;

  Ok(CreateTagResult {
    oid: oid.to_string(),
  })
}

#[napi]
pub fn delete_tag(name: String, context: GitContext) -> anyhow::Result<()> {
  let repo = Repository::open(context.dir)?;
  repo.tag_delete(&name)?;

  Ok(())
}
