use git2::Repository;

use crate::context::GitContext;

#[napi]
pub fn remove_ref(git_ref: String, context: GitContext) -> crate::Result<()> {
  let repo = Repository::open(context.dir)?;
  let mut obj = repo.resolve_reference_from_short_name(&git_ref)?;

  obj.delete()?;

  Ok(())
}
