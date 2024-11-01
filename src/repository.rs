use std::ffi::CString;

use anyhow::Error;
use git2::{Index, IndexEntry, Repository};

use crate::context::GitContext;

#[napi]
pub fn get_sha(git_ref: String, context: GitContext) -> anyhow::Result<String> {
  let repo = Repository::open(context.dir)?;
  let obj = repo.revparse_single(&git_ref)?;

  Ok(obj.id().to_string())
}

#[napi]
pub fn get_head_sha(context: GitContext) -> anyhow::Result<String> {
  let head = String::from("HEAD");

  get_sha(head, context)
}

#[napi]
pub fn get_git_root_path(context: GitContext) -> anyhow::Result<String> {
  let repo = Repository::open(context.dir)?;
  let path = repo.path().to_str().ok_or(Error::msg("Repository path is not valid"))?;

  Ok(String::from(path))
}

#[napi]
pub fn has_merge_conflicts(ref1: String, ref2: String, context: GitContext) -> anyhow::Result<bool> {
  let tree = get_merge_tree(ref1, ref2, context)?;

  Ok(tree.has_conflicts())
}

#[napi(object)]
pub struct Conflict {
  pub ancestor: Option<String>,
  pub our: Option<String>,
  pub their: Option<String>,
}

#[napi]
pub fn get_conflicting_files(ref1: String, ref2: String, context: GitContext) -> anyhow::Result<Vec<Conflict>> {
  let tree = get_merge_tree(ref1, ref2, context)?;
  let conflicts_list = tree.conflicts();

  let mut files = vec![];

  for conflicts in conflicts_list {
    for conflict in conflicts {
      let conflict = conflict?;

      let ancestor = conflict.ancestor.and_then(parse_index_entry);
      let our = conflict.our.and_then(parse_index_entry);
      let their = conflict.their.and_then(parse_index_entry);

      let conflict = Conflict { ancestor, our, their };

      files.push(conflict);
    }
  }

  Ok(files)
}

fn parse_index_entry(entry: IndexEntry) -> Option<String> {
  let str = CString::new(&entry.path[..]).ok()?;
  let str = str.to_str().ok()?;

  Some(str.to_owned())
}

pub fn get_merge_tree(ref1: String, ref2: String, context: GitContext) -> anyhow::Result<Index> {
  let repo = Repository::open(context.dir)?;
  let obj1 = repo.revparse_single(&ref1)?;
  let obj2 = repo.revparse_single(&ref2)?;

  let commit1 = obj1.peel_to_commit()?;
  let commit2 = obj2.peel_to_commit()?;

  let base = repo
    .merge_base(commit1.id(), commit2.id())
    .and_then(|oid| repo.find_commit(oid))?;

  let merge_tree = repo.merge_trees(&base.tree()?, &commit1.tree()?, &commit2.tree()?, None)?;

  Ok(merge_tree)
}
