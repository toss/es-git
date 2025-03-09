use crate::blob::{Blob, BlobInner};
use crate::commit::{Commit, CommitInner};
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

#[napi]
/// An enumeration all possible kinds objects may have.
pub enum ObjectType {
  /// Any kind of git object
  Any,
  /// An object which corresponds to a git commit
  Commit,
  /// An object which corresponds to a git tree
  Tree,
  /// An object which corresponds to a git blob
  Blob,
  /// An object which corresponds to a git tag
  Tag,
}

impl From<git2::ObjectType> for ObjectType {
  fn from(value: git2::ObjectType) -> Self {
    match value {
      git2::ObjectType::Any => ObjectType::Any,
      git2::ObjectType::Commit => ObjectType::Commit,
      git2::ObjectType::Tree => ObjectType::Tree,
      git2::ObjectType::Blob => ObjectType::Blob,
      git2::ObjectType::Tag => ObjectType::Tag,
    }
  }
}

impl From<ObjectType> for git2::ObjectType {
  fn from(value: ObjectType) -> Self {
    match value {
      ObjectType::Any => git2::ObjectType::Any,
      ObjectType::Commit => git2::ObjectType::Commit,
      ObjectType::Tree => git2::ObjectType::Tree,
      ObjectType::Blob => git2::ObjectType::Blob,
      ObjectType::Tag => git2::ObjectType::Tag,
    }
  }
}

pub(crate) enum ObjectInner {
  Repo(SharedReference<Repository, git2::Object<'static>>),
  Owned(git2::Object<'static>),
}

impl Deref for ObjectInner {
  type Target = git2::Object<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(inner) => inner.deref(),
      Self::Owned(inner) => inner,
    }
  }
}

#[napi]
/// A structure to represent a Git [Object][1]
/// @hideconstructor
///
/// [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct GitObject {
  pub(crate) inner: ObjectInner,
}

#[napi]
impl GitObject {
  #[napi]
  /// Get the id (SHA1) of a repository object
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi(js_name = "type")]
  /// Get the object type of object.
  ///
  /// If the type is unknown, then `null` is returned.
  pub fn kind(&self) -> Option<ObjectType> {
    self.inner.kind().map(|x| x.into())
  }

  #[napi]
  /// Recursively peel an object until an object of the specified type is met.
  ///
  /// If you pass `Any` as the target type, then the object will be
  /// peeled until the type changes (e.g. a tag will be chased until the
  /// referenced object is no longer a tag).
  pub fn peel(&self, obj_type: ObjectType) -> crate::Result<Self> {
    let git_object = self.inner.peel(obj_type.into())?;
    let object = Self {
      inner: ObjectInner::Owned(git_object),
    };
    Ok(object)
  }

  #[napi]
  /// Recursively peel an object until a commit is found
  pub fn peel_to_commit(&self) -> crate::Result<Commit> {
    let git_commit = self.inner.peel_to_commit()?;
    let commit = Commit {
      inner: CommitInner::Owned(git_commit),
    };
    Ok(commit)
  }

  #[napi]
  /// Recursively peel an object until a blob is found
  pub fn peel_to_blob(&self, env: Env, this: Reference<GitObject>) -> crate::Result<Blob> {
    let blob = this.share_with(env, |obj| {
      obj
        .inner
        .peel_to_blob()
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Blob {
      inner: BlobInner::GitObject(blob),
    })
  }

  #[napi]
  /// Attempt to view this object as a commit.
  ///
  /// Returns `null` if the object is not actually a commit.
  pub fn as_commit(&self) -> Option<Commit> {
    self.inner.as_commit().map(|x| Commit {
      inner: CommitInner::Owned(x.clone()),
    })
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  ///
  /// Returns `null` if the object does not exist.
  pub fn find_object(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<GitObject> {
    self.get_object(this, env, oid).ok()
  }

  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  pub fn get_object(&self, this: Reference<Repository>, env: Env, oid: String) -> crate::Result<GitObject> {
    let object = this.share_with(env, |repo| {
      repo
        .inner
        .find_object_by_prefix(&oid, None)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(GitObject {
      inner: ObjectInner::Repo(object),
    })
  }
}
