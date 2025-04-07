use crate::blob::{Blob, BlobInner};
use crate::commit::{Commit, CommitInner};
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

#[napi(string_enum)]
/// - `Any` : Any kind of git object
/// - `Commit` : An object which corresponds to a git commit
/// - `Tree` : An object which corresponds to a git tree
/// - `Blob` : An object which corresponds to a git blob
/// - `Tag` : An object which corresponds to a git tag
pub enum ObjectType {
  Any,
  Commit,
  Tree,
  Blob,
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
/// A class to represent a git [object][1].
///
/// [1]: https://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct GitObject {
  pub(crate) inner: ObjectInner,
}

#[napi]
impl GitObject {
  #[napi]
  /// Get the id (SHA1) of a repository object.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns ID(SHA1) of a repository object.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi(js_name = "type")]
  /// Get the object type of object.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   type(): ObjectType | null;
  /// }
  /// ```
  ///
  /// @returns If the type is unknown, then `null` is returned.
  pub fn kind(&self) -> Option<ObjectType> {
    self.inner.kind().map(|x| x.into())
  }

  #[napi]
  /// Recursively peel an object until an object of the specified type is met.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   peel(objType: ObjectType): GitObject;
  /// }
  /// ```
  ///
  /// @param {ObjectType} objType - If you pass `Any` as the target type, then the object will be
  /// peeled until the type changes (e.g. a tag will be chased until the
  /// referenced object is no longer a tag).
  ///
  /// @returns Git object which recursively peeled.
  pub fn peel(&self, obj_type: ObjectType) -> crate::Result<Self> {
    let git_object = self.inner.peel(obj_type.into())?;
    let object = Self {
      inner: ObjectInner::Owned(git_object),
    };
    Ok(object)
  }

  #[napi]
  /// Recursively peel an object until a commit is found.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   peelToCommit(): Commit;
  /// }
  /// ```
  ///
  /// @returns Git commit.
  pub fn peel_to_commit(&self) -> crate::Result<Commit> {
    let git_commit = self.inner.peel_to_commit()?;
    let commit = Commit {
      inner: CommitInner::Owned(git_commit),
    };
    Ok(commit)
  }

  #[napi]
  /// Recursively peel an object until a blob is found.
  ///
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   peelToBlob(): Blob;
  /// }
  /// ```
  ///
  /// @returns Git blob.
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
  /// @category GitObject/Methods
  /// @signature
  /// ```ts
  /// class GitObject {
  ///   asCommit(): Commit | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if the object is not actually a commit.
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
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findObject(oid: string): GitObject | null;
  /// }
  /// ```
  ///
  /// @param {string} oid - Git object ID(SHA1) to lookup.
  /// @returns Git object. Returns `null` if the object does not exist.
  pub fn find_object(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<GitObject> {
    self.get_object(this, env, oid).ok()
  }

  #[napi]
  /// Lookup a reference to one of the objects in a repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getObject(oid: string): GitObject;
  /// }
  /// ```
  ///
  /// @param {string} oid - Git object ID(SHA1) to lookup.
  /// @returns Git object.
  /// @throws Throws error if the object does not exist.
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
