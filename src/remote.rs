use git2::Repository;

use crate::context::GitContext;

#[napi]
pub fn get_remote_url(name: String, context: GitContext) -> anyhow::Result<Option<String>> {
  let repo = Repository::open(context.dir)?;
  let remote = repo.find_remote(&name)?;
  let url = remote.url().map(|x| x.to_string());
  Ok(url)
}
