use crate::commit::Commit;
use crate::object::{GitObject, ObjectInner, ObjectType};
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;
use std::path::Path;

#[napi(string_enum)]
/// A binary indicator of whether a tree walk should be performed in pre-order
/// or post-order.
pub enum TreeWalkMode {
  PreOrder,
  PostOrder,
}

impl From<TreeWalkMode> for git2::TreeWalkMode {
  fn from(value: TreeWalkMode) -> Self {
    match value {
      TreeWalkMode::PreOrder => git2::TreeWalkMode::PreOrder,
      TreeWalkMode::PostOrder => git2::TreeWalkMode::PostOrder,
    }
  }
}

pub(crate) enum TreeInner {
  Repo(SharedReference<Repository, git2::Tree<'static>>),
  Reference(SharedReference<crate::reference::Reference, git2::Tree<'static>>),
  Commit(SharedReference<Commit, git2::Tree<'static>>),
}

impl Deref for TreeInner {
  type Target = git2::Tree<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(inner) => inner.deref(),
      Self::Reference(inner) => inner.deref(),
      Self::Commit(inner) => inner.deref(),
    }
  }
}

#[napi]
/// A class to represent a git [tree][1].
/// @hideconstructor
///
/// [1]: https://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct Tree {
  pub(crate) inner: TreeInner,
}

#[napi]
impl Tree {
  #[napi]
  /// Get the id (SHA1) of a repository object.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the number of entries listed in this tree.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  #[napi]
  /// Return `true` if there is no entry.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  /// Returns an iterator over the entries in this tree.
  pub fn iter(&self, this: Reference<Tree>, env: Env) -> crate::Result<TreeIter> {
    let inner = this.share_with(env, |tree| Ok(tree.inner.iter()))?;
    Ok(TreeIter { inner })
  }

  #[napi(ts_args_type = "mode: TreeWalkMode, callback: (entry: TreeEntry) => number")]
  /// Traverse the entries in a tree and its subtrees in post or pre-order.
  /// The callback function will be run on each node of the tree that's
  /// walked. The return code of this function will determine how the walk
  /// continues.
  ///
  /// libgit2 requires that the callback be an integer, where 0 indicates a
  /// successful visit, 1 skips the node, and -1 aborts the traversal completely.

  /// See [libgit2 documentation][1] for more information.
  ///
  /// [1]: https://libgit2.org/libgit2/#HEAD/group/tree/git_tree_walk
  pub fn walk(&self, env: Env, mode: TreeWalkMode, callback: Function<'static, TreeEntry, i32>) -> crate::Result<()> {
    let mut git2_cb = move |_: &str, git2_entry: &git2::TreeEntry<'_>| {
      let entry = TreeEntry {
        inner: TreeEntryInner::Owned(git2_entry.to_owned()),
      };
      callback
        .create_ref()
        .unwrap()
        .borrow_back(&env)
        .and_then(|callback| callback.call(entry))
        .unwrap()
    };
    self.inner.walk(mode.into(), &mut git2_cb)?;
    Ok(())
  }

  #[napi]
  /// Lookup a tree entry by SHA value.
  pub fn get_id(&self, this: Reference<Tree>, env: Env, id: String) -> crate::Result<Option<TreeEntry>> {
    let oid = git2::Oid::from_str(&id)?;
    let entry = this
      .share_with(env, |tree| {
        tree
          .inner
          .get_id(oid)
          .ok_or(Error::new(Status::InvalidArg, "tree entry not found"))
      })
      .ok()
      .map(|entry| TreeEntry {
        inner: TreeEntryInner::Tree(entry),
      });
    Ok(entry)
  }

  #[napi]
  /// Lookup a tree entry by its position in the tree.
  pub fn get(&self, this: Reference<Tree>, env: Env, index: u32) -> crate::Result<Option<TreeEntry>> {
    let entry = this
      .share_with(env, |tree| {
        tree
          .inner
          .get(index as usize)
          .ok_or(Error::new(Status::InvalidArg, "tree entry not found"))
      })
      .ok()
      .map(|entry| TreeEntry {
        inner: TreeEntryInner::Tree(entry),
      });
    Ok(entry)
  }

  #[napi]
  /// Lookup a tree entry by its filename.
  pub fn get_name(&self, this: Reference<Tree>, env: Env, filename: String) -> crate::Result<Option<TreeEntry>> {
    let entry = this
      .share_with(env, |tree| {
        tree
          .inner
          .get_name(&filename)
          .ok_or(Error::new(Status::InvalidArg, "tree entry not found"))
      })
      .ok()
      .map(|entry| TreeEntry {
        inner: TreeEntryInner::Tree(entry),
      });
    Ok(entry)
  }

  #[napi]
  /// Retrieve a tree entry contained in a tree or in any of its subtrees,
  /// given its relative path.
  pub fn get_path(&self, this: Reference<Tree>, env: Env, path: String) -> crate::Result<Option<TreeEntry>> {
    let entry = this
      .share_with(env, |tree| {
        tree
          .inner
          .get_path(Path::new(&path))
          .map_err(crate::Error::from)
          .map_err(|e| e.into())
      })
      .ok()
      .map(|entry| TreeEntry {
        inner: TreeEntryInner::Tree(entry),
      });
    Ok(entry)
  }

  #[napi]
  /// Casts this Tree to be usable as an `GitObject`.
  pub fn as_object(&self) -> GitObject {
    GitObject {
      inner: ObjectInner::Owned(self.inner.as_object().clone()),
    }
  }
}

#[napi(iterator)]
/// An iterator over the entries in a tree.
///
/// @hideconstructor
pub struct TreeIter {
  pub(crate) inner: SharedReference<Tree, git2::TreeIter<'static>>,
}

#[napi]
impl Generator for TreeIter {
  type Yield = TreeEntry;
  type Return = ();
  type Next = ();

  fn next(&mut self, _value: Option<()>) -> Option<Self::Yield> {
    self.inner.next().map(|e| TreeEntry {
      inner: TreeEntryInner::Owned(e),
    })
  }
}

pub(crate) enum TreeEntryInner {
  Tree(SharedReference<Tree, git2::TreeEntry<'static>>),
  Owned(git2::TreeEntry<'static>),
}

impl Deref for TreeEntryInner {
  type Target = git2::TreeEntry<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Tree(inner) => inner.deref(),
      Self::Owned(inner) => inner,
    }
  }
}

#[napi]
/// A class representing an entry inside of a tree. An entry is borrowed
/// from a tree.
///
/// @hideconstructor
pub struct TreeEntry {
  pub(crate) inner: TreeEntryInner,
}

#[napi]
impl TreeEntry {
  #[napi]
  /// Get the id of the object pointed by the entry.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the filename of a tree entry.
  ///
  /// Throws error if the name is not valid utf-8.
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes())?;
    Ok(name.to_string())
  }

  #[napi(js_name = "type")]
  /// Get the type of the object pointed by the entry.
  pub fn kind(&self) -> Option<ObjectType> {
    self.inner.kind().map(|x| x.into())
  }

  #[napi]
  /// Get the UNIX file attributes of a tree entry.
  pub fn filemode(&self) -> i32 {
    self.inner.filemode()
  }

  #[napi]
  /// Convert a tree entry to the object it points to.
  pub fn to_object(&self, env: Env, repo: Reference<Repository>) -> crate::Result<GitObject> {
    let object = repo.share_with(env, |repo| {
      self
        .inner
        .to_object(&repo.inner)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(GitObject {
      inner: ObjectInner::Repo(object),
    })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  pub fn get_tree(&self, this: Reference<Repository>, env: Env, oid: String) -> crate::Result<Tree> {
    let oid = git2::Oid::from_str(&oid)?;
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_tree(oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Tree {
      inner: TreeInner::Repo(inner),
    })
  }

  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  ///
  /// If it does not exist, returns `null`.
  pub fn find_tree(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<Tree> {
    self.get_tree(this, env, oid).ok()
  }
}
