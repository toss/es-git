use crate::repository::Repository;
use crate::signature::Signature;
use chrono::{DateTime, Utc};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

pub(crate) enum CommitInner {
  Repo(SharedReference<Repository, git2::Commit<'static>>),
}

impl Deref for CommitInner {
  type Target = git2::Commit<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
    }
  }
}

#[napi]
pub struct Commit {
  pub(crate) inner: CommitInner,
}

#[napi]
/// /// A structure to represent a git commit
impl Commit {
  #[napi]
  /// Get the id (SHA1) of a repository commit
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  /// Get the author of this commit.
  pub fn author(&self) -> crate::Result<Signature> {
    let signature = Signature::try_from(self.inner.author())?;
    Ok(signature)
  }

  #[napi]
  /// Get the committer of this commit.
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
  /// Throws error if the message is not valid utf-8
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
  /// The first element of the tuple is the time, in seconds, since the epoch.
  /// The second element is the offset, in minutes, of the time zone of the
  /// committer's preferred time zone.
  pub fn time(&self) -> crate::Result<DateTime<Utc>> {
    let time = DateTime::from_timestamp(self.inner.time().seconds(), 0).ok_or(crate::Error::InvalidTime)?;
    Ok(time)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Lookup a reference to one of the commits in a repository.
  pub fn find_commit(&self, this: Reference<Repository>, env: Env, oid: String) -> crate::Result<Commit> {
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
}
