use std::ops::Deref;
use crate::commit::Commit;
use crate::reference::Reference as GitReference;
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
/// A structure to represent an annotated commit, the input to merge and rebase.
///
/// An annotated commit contains information about how it was looked up, which
/// may be useful for functions like merge or rebase to provide context to the
/// operation.
pub struct AnnotatedCommit {
  pub(crate) from_ref: bool,
  pub(crate) inner: SharedReference<Repository, git2::AnnotatedCommit<'static>>,
}

impl Deref for AnnotatedCommit {
  type Target = git2::AnnotatedCommit<'static>;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[napi]
impl AnnotatedCommit {
  #[napi]
  /// Gets the commit ID that the given Annotated Commit refers to.
  ///
  /// @category AnnotatedCommit/Methods
  /// @signature
  /// ```ts
  /// class AnnotatedCommit {
  ///   id(): string;
  /// }
  /// ```
  ///
  /// @returns The commit ID that this Annotated Commit refers to.
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the refname that the given Annotated Commit refers to.
  ///
  /// @category AnnotatedCommit/Methods
  /// @signature
  /// ```ts
  /// class AnnotatedCommit {
  ///   refname(): string | null;
  /// }
  /// ```
  ///
  /// @returns The refname that this Annotated Commit refers to. If this created from a reference,
  /// the return value is `null`.
  /// @throws Throws error if the refname is not valid utf-8.
  pub fn refname(&self) -> crate::Result<Option<String>> {
    // If you created an Annotated Commit from a commit, there is no refname.
    // Since the `.refname()` method will cause a panic in `.unwrap()`, so we have to check
    // that this annotated commit is created from a reference explicitly.
    if self.from_ref {
      let refname = std::str::from_utf8(self.inner.refname_bytes())?;
      return Ok(Some(refname.to_string()));
    }
    Ok(None)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Creates an Annotated Commit from the given commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getAnnotatedCommit(commit: Commit): AnnotatedCommit;
  /// }
  /// ```
  ///
  /// @param {Commit} commit - Commit to creates a Annotated Commit.
  /// @returns An Annotated Commit created from commit.
  pub fn get_annotated_commit(
    &self,
    commit: &Commit,
    this: Reference<Repository>,
    env: Env,
  ) -> crate::Result<AnnotatedCommit> {
    let oid = commit.inner.id();
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_annotated_commit(oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(AnnotatedCommit { from_ref: false, inner })
  }

  #[napi]
  /// Creates a Annotated Commit from the given reference.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getAnnotatedCommitFromReference(reference: Reference): AnnotatedCommit;
  /// }
  /// ```
  ///
  /// @param {Reference} reference - Reference to creates a Annotated Commit.
  /// @returns An Annotated Commit created from reference.
  pub fn get_annotated_commit_from_reference(
    &self,
    reference: &GitReference,
    this: Reference<Repository>,
    env: Env,
  ) -> crate::Result<AnnotatedCommit> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .reference_to_annotated_commit(&reference.inner)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(AnnotatedCommit { from_ref: true, inner })
  }

  #[napi]
  /// Creates a Annotated Commit from `FETCH_HEAD`.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getAnnotatedCommitFromFetchHead(
  ///     branchName: string,
  ///     remoteUrl: string,
  ///     id: string,
  ///   ): AnnotatedCommit;
  /// }
  /// ```
  ///
  /// @param {String} branchName - Name of the remote branch.
  /// @param {String} remoteUrl - Url of the remote.
  /// @param {String} id - The commit object id of the remote branch.
  /// @returns An Annotated Commit created from `FETCH_HEAD`.
  pub fn get_annotated_commit_from_fetch_head(
    &self,
    branch_name: String,
    remote_url: String,
    id: String,
    this: Reference<Repository>,
    env: Env,
  ) -> crate::Result<AnnotatedCommit> {
    let oid = git2::Oid::from_str(&id)?;
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .annotated_commit_from_fetchhead(&branch_name, &remote_url, &oid)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(AnnotatedCommit { from_ref: true, inner })
  }
}
