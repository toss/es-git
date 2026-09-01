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

#[napi(object)]
#[derive(Default)]
pub struct AmendOptions {
  /// If not NULL, name of the reference that will be updated to point to this commit.
  /// If the reference is not direct, it will be resolved to a direct reference.
  /// Use "HEAD" to update the HEAD of the current branch and make it point to this commit.
  ///
  /// If the reference doesn't exist yet, it will be created.
  /// If it does exist, the first parent must be the tip of this branch.
  pub update_ref: Option<String>,
  /// Signature for author.
  pub author: Option<SignaturePayload>,
  /// Signature for committer.
  pub committer: Option<SignaturePayload>,
  /// Full message for this commit
  pub message: Option<String>,
  /// The encoding for the message in the commit, represented with a standard encoding name.
  /// E.g. "UTF-8".
  /// If NULL, no encoding header is written and UTF-8 is assumed.
  pub message_encoding: Option<String>,
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
  /// Get the id of the tree pointed to by this commit.
  ///
  /// No attempts are made to fetch an object from the ODB.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   treeId(): string;
  /// }
  /// ```
  ///
  /// @returns Get the id of the tree pointed to by a commit.
  pub fn tree_id(&self) -> String {
    self.inner.tree_id().to_string()
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

  #[napi]
  /// Amend this existing commit with all non-nullable values
  ///
  /// This creates a new commit that is exactly the same as the old commit,
  /// except that any non-nullable values will be updated. The new commit has
  /// the same parents as the old commit.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   amend(options?: AmendOptions, tree?: Tree): string;
  /// }
  /// ```
  ///
  /// @param {AmendOptions} [options] - Options for amending commit.
  /// @param {Tree} [tree] - Tree to use for amending commit.
  /// @returns ID(SHA1) of amended commit.
  pub fn amend(&self, options: Option<AmendOptions>, tree: Option<&Tree>) -> crate::Result<String> {
    let opts = options.unwrap_or_default();
    let update_ref = opts.update_ref;
    let author = opts
      .author
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok());
    let committer = opts
      .committer
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok());
    let message = opts.message;
    let message_encoding = opts.message_encoding;

    let oid = self.inner.amend(
      update_ref.as_deref(),
      author.as_ref(),
      committer.as_ref(),
      message_encoding.as_deref(),
      message.as_deref(),
      tree.map(|x| x.inner.deref()),
    )?;
    Ok(oid.to_string())
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
      // `commit_signed()` only writes the commit object, unlike `commit()` which
      // also validates parents against the ref tip and moves the ref inside
      // libgit2. Mirror that behavior here so both paths behave the same.
      let update_target = update_ref
        .as_deref()
        .map(|name| self.resolve_commit_update_ref(name))
        .transpose()?;

      let parents = parents.unwrap_or_default();
      if let Some(CommitUpdateRef::Resolved(reference)) = &update_target {
        // Validate before writing the object so a failure leaves no orphaned
        // commit in the odb, matching libgit2's ordering.
        let first_parent = parents.first().map(|parent| parent.id());
        if reference.target() != first_parent {
          return Err(
            git2::Error::new(
              git2::ErrorCode::Modified,
              git2::ErrorClass::Object,
              "failed to create commit: current tip is not the first parent",
            )
            .into(),
          );
        }
      }

      let commit_content = self.inner.commit_create_buffer(
        &author,
        &committer,
        &message,
        &tree.inner,
        &parents.iter().collect::<Vec<_>>(),
      )?;

      let commit_content_str = std::str::from_utf8(&commit_content)?.to_string();

      let oid = self
        .inner
        .commit_signed(&commit_content_str, &signature_str, signature_field.as_deref())?;

      if let Some(update_target) = update_target {
        self.update_ref_for_commit(update_target, oid)?;
      }

      oid
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

/// Resolution of an `updateRef` name for commit creation, mirroring how libgit2
/// treats the ref on its unsigned commit path (`git_commit__create_internal`).
enum CommitUpdateRef<'repo> {
  /// The name resolved to an existing direct reference.
  Resolved(git2::Reference<'repo>),
  /// The name (or the branch its symbolic ref points to) does not exist yet;
  /// a reference with this name should be created.
  Create(String),
}

impl Repository {
  /// Mirrors `git_reference_lookup_resolved` as used by libgit2 when creating
  /// a commit: a missing ref or a symbolic ref to an unborn branch (e.g. HEAD
  /// in a fresh repository) is not an error, anything else propagates.
  fn resolve_commit_update_ref(&self, name: &str) -> crate::Result<CommitUpdateRef<'_>> {
    match self.inner.find_reference(name) {
      Ok(reference) => match reference.resolve() {
        Ok(resolved) => Ok(CommitUpdateRef::Resolved(resolved)),
        Err(e) if e.code() == git2::ErrorCode::NotFound => {
          let target = reference.symbolic_target().unwrap_or(name).to_string();
          Ok(CommitUpdateRef::Create(target))
        }
        Err(e) => Err(e.into()),
      },
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(CommitUpdateRef::Create(name.to_string())),
      Err(e) => Err(e.into()),
    }
  }

  /// Points the resolved ref at the new commit, mirroring
  /// `git_reference__update_for_commit`: same reflog message format, and
  /// updating an existing ref asserts its target has not moved since it was
  /// resolved. The reflog identity falls back to the repository default since
  /// git2 does not accept a signature on reference updates.
  fn update_ref_for_commit(&self, target: CommitUpdateRef<'_>, oid: git2::Oid) -> crate::Result<()> {
    let commit = self.inner.find_commit(oid)?;
    let commit_type = match commit.parent_count() {
      0 => " (initial)",
      1 => "",
      _ => " (merge)",
    };
    let reflog_message = format!("commit{}: {}", commit_type, commit.summary().unwrap_or_default());
    match target {
      CommitUpdateRef::Resolved(mut reference) => {
        reference.set_target(oid, &reflog_message)?;
      }
      CommitUpdateRef::Create(name) => {
        // Create-only (no force), like libgit2's `git_reference__update_terminal`:
        // if the ref appeared in the meantime this errors instead of clobbering it.
        self.inner.reference(&name, oid, false, &reflog_message)?;
      }
    }
    Ok(())
  }
}
