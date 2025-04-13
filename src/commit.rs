use crate::object::{GitObject, ObjectInner};
use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use crate::tree::{Tree, TreeInner};
use chrono::{DateTime, Utc};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

#[napi(object)]
pub struct CommitOptions {
  pub update_ref: Option<String>,
  /// Signature for author.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub author: Option<SignaturePayload>,
  /// Signature for commiter.
  ///
  /// If not provided, the default signature of the repository will be used.
  /// If there is no default signature set for the repository, an error will occur.
  pub committer: Option<SignaturePayload>,
  pub parents: Option<Vec<String>>,
  /// GPG signature string for signed commits.
  ///
  /// If provided, this will create a signed commit.
  pub signature: Option<String>,
  /// Custom signature field name.
  ///
  /// If not provided, the default signature field (gpgsig) will be used.
  pub signature_field: Option<String>,
}

pub(crate) enum CommitInner {
  Repo(SharedReference<Repository, git2::Commit<'static>>),
  Owned(git2::Commit<'static>),
}

impl Deref for CommitInner {
  type Target = git2::Commit<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
      Self::Owned(commit) => commit,
    }
  }
}

#[napi]
/// A class to represent a git commit.
pub struct Commit {
  pub(crate) inner: CommitInner,
}

#[napi]
impl Commit {
  #[napi]
  /// Get the id (SHA1) of a repository commit
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns ID(SHA1) of a repository commit.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the author of this commit.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   author(): Signature;
  /// }
  /// ```
  ///
  /// @returns Author signature of this commit.
  pub fn author(&self) -> crate::Result<Signature> {
    let signature = Signature::try_from(self.inner.author())?;
    Ok(signature)
  }

  #[napi]
  /// Get the committer of this commit.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   committer(): Signature;
  /// }
  /// ```
  ///
  /// @returns Committer signature of this commit.
  pub fn committer(&self) -> crate::Result<Signature> {
    let signature = Signature::try_from(self.inner.committer())?;
    Ok(signature)
  }

  #[napi]
  /// Get the full message of a commit.
  ///
  /// The returned message will be slightly prettified by removing any
  /// potential leading newlines.
  ///
  /// Throws error if the message is not valid utf-8.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   message(): string;
  /// }
  /// ```
  ///
  /// @returns Full message of this commit.
  /// @throws If the message is not valid utf-8.
  pub fn message(&self) -> crate::Result<String> {
    let message = std::str::from_utf8(self.inner.message_raw_bytes())?.to_string();
    Ok(message)
  }

  #[napi]
  /// Get the short "summary" of the git commit message.
  ///
  /// The returned message is the summary of the commit, comprising the first
  /// paragraph of the message with whitespace trimmed and squashed.
  ///
  /// Throws error if the summary is not valid utf-8.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   summary(): string | null;
  /// }
  /// ```
  ///
  /// @returns Short summary of this commit message.
  /// @throws If the summary is not valid utf-8.
  pub fn summary(&self) -> crate::Result<Option<String>> {
    let summary = match self.inner.summary_bytes() {
      Some(bytes) => Some(std::str::from_utf8(bytes)?.to_string()),
      None => None,
    };
    Ok(summary)
  }

  #[napi]
  /// Get the long "body" of the git commit message.
  ///
  /// The returned message is the body of the commit, comprising everything
  /// but the first paragraph of the message. Leading and trailing whitespaces
  /// are trimmed.
  ///
  /// Throws error if the summary is not valid utf-8.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   body(): string | null;
  /// }
  /// ```
  ///
  /// @returns Long body of this commit message.
  /// @throws If the body is not valid utf-8.
  pub fn body(&self) -> crate::Result<Option<String>> {
    let body = match self.inner.body_bytes() {
      Some(bytes) => Some(std::str::from_utf8(bytes)?.to_string()),
      None => None,
    };
    Ok(body)
  }

  #[napi]
  /// Get the commit time (i.e. committer time) of a commit.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   time(): Date;
  /// }
  /// ```
  ///
  /// @returns Commit time of a commit.
  pub fn time(&self) -> crate::Result<DateTime<Utc>> {
    let time = DateTime::from_timestamp(self.inner.time().seconds(), 0).ok_or(crate::Error::InvalidTime)?;
    Ok(time)
  }

  #[napi]
  /// Get the tree pointed to by a commit.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   tree(): Tree;
  /// }
  /// ```
  ///
  /// @returns Tree pointed to by a commit.
  pub fn tree(&self, this: Reference<Commit>, env: Env) -> crate::Result<Tree> {
    let tree = this.share_with(env, |commit| {
      commit.inner.tree().map_err(crate::Error::from).map_err(|e| e.into())
    })?;
    Ok(Tree {
      inner: TreeInner::Commit(tree),
    })
  }

  #[napi]
  /// Casts this Commit to be usable as an `GitObject`.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   asObject(): GitObject;
  /// }
  /// ```
  ///
  /// @returns `GitObject` that casted from this commit.
  pub fn as_object(&self) -> GitObject {
    let obj = self.inner.as_object().clone();
    GitObject {
      inner: ObjectInner::Owned(obj),
    }
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reference to one of the commits in a repository.
  ///
  /// Returns `null` if the commit does not exist.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findCommit(oid: string): Commit | null;
  /// }
  /// ```
  /// @param {string} oid - Commit ID(SHA1) to lookup.
  /// @returns Commit instance found by oid. Returns `null` if the commit does not exist.
  pub fn find_commit(&self, this: Reference<Repository>, env: Env, oid: String) -> Option<Commit> {
    self.get_commit(this, env, oid).ok()
  }

  #[napi]
  /// Lookup a reference to one of the commits in a repository.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getCommit(oid: string): Commit;
  /// }
  /// ```
  ///
  /// @param {string} oid - Commit ID(SHA1) to lookup.
  /// @returns Commit instance found by oid.
  /// @throws Throws error if the commit does not exist.
  pub fn get_commit(&self, this: Reference<Repository>, env: Env, oid: String) -> crate::Result<Commit> {
    let commit = this.share_with(env, |repo| {
      repo
        .inner
        .find_commit_by_prefix(&oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Commit {
      inner: CommitInner::Repo(commit),
    })
  }

  #[napi]
  /// Create new commit in the repository.
  ///
  /// If the `updateRef` is not `null`, name of the reference that will be
  /// updated to point to this commit. If the reference is not direct, it will
  /// be resolved to a direct reference. Use "HEAD" to update the HEAD of the
  /// current branch and make it point to this commit. If the reference
  /// doesn't exist yet, it will be created. If it does exist, the first
  /// parent must be the tip of this branch.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   commit(tree: Tree, message: string, options?: CommitOptions | null | undefined): string;
  /// }
  /// ```
  ///
  /// @returns ID(SHA1) of created commit.
  pub fn commit(&self, tree: &Tree, message: String, options: Option<CommitOptions>) -> crate::Result<String> {
    let (update_ref, author, committer, parents, signature, signature_field) = match options {
      Some(opts) => {
        let update_ref = opts.update_ref;
        let author = opts.author.and_then(|x| Signature::try_from(x).ok());
        let committer = opts.committer.and_then(|x| Signature::try_from(x).ok());
        let parents = match opts.parents {
          Some(parents) => {
            let commits: crate::Result<Vec<git2::Commit>> = parents
              .iter()
              .map(|x| self.inner.find_commit_by_prefix(x).map_err(crate::Error::from))
              .collect();
            Some(commits?)
          }
          None => None,
        };
        let signature = opts.signature;
        let signature_field = opts.signature_field;
        (update_ref, author, committer, parents, signature, signature_field)
      }
      None => (None, None, None, None, None, None),
    };
    let author = author
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;
    let committer = committer
      .and_then(|x| git2::Signature::try_from(x).ok())
      .or_else(|| self.inner.signature().ok())
      .ok_or(crate::Error::SignatureNotFound)?;

    let oid = if let Some(signature_str) = signature {
      let commit_content = self.inner.commit_create_buffer(
        &author,
        &committer,
        &message,
        &tree.inner,
        &parents.unwrap_or_default().iter().collect::<Vec<_>>(),
      )?;

      let commit_content_str = std::str::from_utf8(&commit_content)?.to_string();

      self
        .inner
        .commit_signed(&commit_content_str, &signature_str, signature_field.as_deref())?
    } else {
      self.inner.commit(
        update_ref.as_deref(),
        &author,
        &committer,
        &message,
        &tree.inner,
        &parents.unwrap_or_default().iter().collect::<Vec<_>>(),
      )?
    };

    Ok(oid.to_string())
  }
}
