use crate::commit::Commit;
use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::{Deref, DerefMut};

pub(crate) enum MailmapInner {
  Repo(SharedReference<Repository, git2::Mailmap>),
  Owned(git2::Mailmap),
}

impl Deref for MailmapInner {
  type Target = git2::Mailmap;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(repo) => repo.deref(),
      Self::Owned(mailmap) => mailmap,
    }
  }
}

impl DerefMut for MailmapInner {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Self::Repo(repo) => repo.deref_mut(),
      Self::Owned(mailmap) => mailmap,
    }
  }
}

/// A wrapper around git2::Mailmap providing Node.js bindings
#[napi]
pub struct Mailmap {
  pub(crate) inner: MailmapInner,
}

#[napi]
impl Mailmap {
  /// Create a mailmap from the contents of a string.
  ///
  /// The format of the string should follow the rules of the mailmap file:
  /// ```
  /// # Comment line (ignored)
  /// Seokju Me <seokju.me@toss.im> Seokju Na <seokju.me@gmail.com>
  /// ```
  ///
  /// @param {string} content - Content of the mailmap file
  /// @returns A new mailmap object or null if operation failed
  #[napi]
  pub fn from_buffer(content: String) -> crate::Result<Option<Mailmap>> {
    match git2::Mailmap::from_buffer(&content) {
      Ok(mailmap) => Ok(Some(Mailmap {
        inner: MailmapInner::Owned(mailmap),
      })),
      Err(e) => Err(e.into()),
    }
  }

  /// Add a new Mailmap entry.
  ///
  /// Maps an author/committer (specified by `replace_name` and `replace_email`)
  /// to the specified real name and email. The `replace_email` is required but
  /// the other parameters can be null.
  ///
  /// If both `replace_name` and `replace_email` are provided, then the entry will
  /// apply to those who match both. If only `replace_name` is provided,
  /// it will apply to anyone with that name, regardless of email. If only
  /// `replace_email` is provided, it will apply to anyone with that email,
  /// regardless of name.
  ///
  /// @param {string} [real_name] - The real name to use, or null
  /// @param {string} [real_email] - The real email to use, or null
  /// @param {string} [replace_name] - The name to replace, or null
  /// @param {string} replace_email - The email to replace
  /// @returns true if the operation succeeded, false otherwise
  #[napi]
  pub fn add_entry(
    &mut self,
    real_name: Option<String>,
    real_email: Option<String>,
    replace_name: Option<String>,
    replace_email: String,
  ) -> bool {
    match &mut self.inner {
      MailmapInner::Repo(repo) => repo
        .add_entry(
          real_name.as_deref(),
          real_email.as_deref(),
          replace_name.as_deref(),
          &replace_email,
        )
        .is_ok(),
      MailmapInner::Owned(mailmap) => mailmap
        .add_entry(
          real_name.as_deref(),
          real_email.as_deref(),
          replace_name.as_deref(),
          &replace_email,
        )
        .is_ok(),
    }
  }

  /// Resolve a signature to its canonical form using a mailmap.
  ///
  /// Returns a new signature with the canonical name and email.
  ///
  /// @param {SignaturePayload} signature - Signature to resolve
  /// @returns The resolved signature with canonical name and email
  #[napi]
  pub fn resolve_signature(&self, signature: SignaturePayload) -> crate::Result<Signature> {
    let git_signature = match &signature.time_options {
      Some(opts) => git2::Signature::new(
        &signature.name,
        &signature.email,
        &git2::Time::new(opts.timestamp, opts.offset.unwrap_or(0)),
      ),
      None => git2::Signature::now(&signature.name, &signature.email),
    }?;

    let resolved = match &self.inner {
      MailmapInner::Repo(repo) => repo.resolve_signature(&git_signature)?,
      MailmapInner::Owned(owned) => owned.resolve_signature(&git_signature)?,
    };

    Signature::try_from(resolved)
  }
}

#[napi]
impl Commit {
  #[napi]
  /// Get the author of this commit, using the mailmap to map it to the canonical name and email.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   authorWithMailmap(mailmap: Mailmap): Signature;
  /// }
  /// ```
  ///
  /// @param {Mailmap} mailmap - The mailmap to use for mapping
  /// @returns Author signature of this commit with mapping applied
  pub fn author_with_mailmap(&self, mailmap: &Mailmap) -> crate::Result<Signature> {
    let git_signature = match &mailmap.inner {
      MailmapInner::Repo(repo) => self.inner.author_with_mailmap(repo)?,
      MailmapInner::Owned(owned) => self.inner.author_with_mailmap(owned)?,
    };
    let signature = Signature::try_from(git_signature)?;
    Ok(signature)
  }

  #[napi]
  /// Get the committer of this commit, using the mailmap to map it to the canonical name and email.
  ///
  /// @category Commit/Methods
  ///
  /// @signature
  /// ```ts
  /// class Commit {
  ///   committerWithMailmap(mailmap: Mailmap): Signature;
  /// }
  /// ```
  ///
  /// @param {Mailmap} mailmap - The mailmap to use for mapping
  /// @returns Committer signature of this commit with mapping applied
  pub fn committer_with_mailmap(&self, mailmap: &Mailmap) -> crate::Result<Signature> {
    let git_signature = match &mailmap.inner {
      MailmapInner::Repo(repo) => self.inner.committer_with_mailmap(repo)?,
      MailmapInner::Owned(owned) => self.inner.committer_with_mailmap(owned)?,
    };
    let signature = Signature::try_from(git_signature)?;
    Ok(signature)
  }
}

#[napi]
impl Repository {
  /// Gets this repository's mailmap.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   mailmap(): Mailmap | null;
  /// }
  /// ```
  ///
  /// @returns The mailmap object if it exists, null otherwise
  #[napi]
  pub fn mailmap(&self, this: Reference<Repository>, env: Env) -> Option<Mailmap> {
    let inner = this
      .share_with(env, |repo| {
        repo.inner.mailmap().map_err(crate::Error::from).map_err(|e| e.into())
      })
      .ok()?;

    Some(Mailmap {
      inner: MailmapInner::Repo(inner),
    })
  }
}
