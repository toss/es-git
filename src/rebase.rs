use crate::annotated_commit::AnnotatedCommit;
use crate::checkout::CheckoutOptions;
use crate::index::Index;
use crate::merge::MergeOptions;
use crate::repository::Repository;
use crate::signature::{Signature, SignaturePayload};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::Deref;

#[napi(object)]
pub struct RebaseCommitOptions {
  /// Signature for author.
  /// To keep the author from the original commit leave this as empty.
  pub author: Option<SignaturePayload>,
  /// Signature for commiter.
  pub committer: SignaturePayload,
  /// To keep the message from the original commit leave this as empty.
  pub message: Option<String>,
}

#[napi]
/// Representation of a rebase
/// Begin the rebase by iterating the returned `Rebase`
/// (e.g., `for (const op of rebase) { ... }` or calling `next()`).
pub struct Rebase {
  pub(crate) inner: SharedReference<Repository, git2::Rebase<'static>>,
}

#[napi]
impl Rebase {
  #[napi]
  /// Gets the count of rebase operations that are to be applied.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   len(): number;
  /// }
  /// ```
  ///
  /// @returns The count of rebase operations.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  #[napi]
  /// Gets the original `HEAD` ref name for merge rebases.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   originHeadName(): string | null;
  /// }
  /// ```
  ///
  /// @returns The original `HEAD` ref name for merge rebases.
  pub fn origin_head_name(&self) -> Option<String> {
    self.inner.orig_head_name().map(|x| x.to_string())
  }

  #[napi]
  /// Gets the original `HEAD` id for merge rebases.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   originHeadId(): string | null;
  /// }
  /// ```
  ///
  /// @returns The original `HEAD` id for merge rebases.
  pub fn origin_head_id(&self) -> Option<String> {
    self.inner.orig_head_id().map(|x| x.to_string())
  }

  #[napi]
  /// Gets the index produced by the last operation, which is the result of
  /// `next()` and which will be committed by the next invocation of
  /// `commit()`. This is useful for resolving conflicts in an in-memory
  /// rebase before committing them.
  ///
  /// This is only applicable for in-memory rebases; for rebases within a
  /// working directory, the changes were applied to the repository's index.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   inmemoryIndex(): Index;
  /// }
  /// ```
  ///
  /// @returns The index produced by the last operation.
  pub fn inmemory_index(&mut self) -> crate::Result<Index> {
    let index = self.inner.inmemory_index().map(|inner| Index { inner })?;
    Ok(index)
  }

  #[napi]
  /// Commits the current patch.  You must have resolved any conflicts that
  /// were introduced during the patch application from the rebase next
  /// invocation.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   commit(options: RebaseCommitOptions): string;
  /// }
  /// ```
  ///
  /// @param {RebaseCommitOptions} options - Options for committing the patch.
  /// @returns The commit ID of the commit that was created.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const rebase = repo.rebase(...);
  /// const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
  /// for (const op of rebase) {
  ///   rebase.commit({ committer: sig });
  /// }
  /// ```
  pub fn commit(&mut self, options: RebaseCommitOptions) -> crate::Result<String> {
    let author = options
      .author
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok());
    let committer = Signature::try_from(options.committer).and_then(git2::Signature::try_from)?;
    let oid = self
      .inner
      .commit(author.as_ref(), &committer, options.message.as_deref())?;
    Ok(oid.to_string())
  }

  #[napi]
  /// Aborts a rebase that is currently in progress, resetting the repository
  /// and working directory to their state before rebase began.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   abort(): void;
  /// }
  /// ```
  pub fn abort(&mut self) -> crate::Result<()> {
    self.inner.abort()?;
    Ok(())
  }

  #[napi]
  /// Finishes a rebase that is currently in progress once all patches have
  /// been applied.
  ///
  /// @category Rebase/Methods
  /// @signature
  /// ```ts
  /// class Rebase {
  ///   finish(signature?: SignaturePayload | undefined | null): void;
  /// }
  /// ```
  ///
  /// @params {SignaturePayload | undefined | null} [signature] - The identity that is finishing the rebase
  pub fn finish(&mut self, signature: Option<SignaturePayload>) -> crate::Result<()> {
    let signature = signature
      .and_then(|x| Signature::try_from(x).ok())
      .and_then(|x| git2::Signature::try_from(x).ok());
    self.inner.finish(signature.as_ref())?;
    Ok(())
  }

  #[napi(js_name = "next")]
  pub fn get_next(&mut self) -> crate::Result<Option<RebaseOperation>> {
    match self.inner.next() {
      Some(x) => {
        let op = RebaseOperation::from(x?);
        Ok(Some(op))
      }
      None => Ok(None),
    }
  }
}

#[napi(object)]
/// A rebase operation
///
/// Describes a single instruction/operation to be performed during the
/// rebase.
pub struct RebaseOperation {
  #[napi(js_name = "type")]
  /// The type of rebase operation
  pub kind: Option<RebaseOperationType>,
  /// The commit ID being cherry-picked. This will be populated for all
  /// operations except those of type `GIT_REBASE_OPERATION_EXEC`.
  pub id: String,
  ///The executable the user has requested be run.  This will only
  /// be populated for operations of type `Exec`.
  pub exec: Option<String>,
}

impl From<git2::RebaseOperation<'_>> for RebaseOperation {
  fn from(value: git2::RebaseOperation<'_>) -> Self {
    let kind = value.kind().map(RebaseOperationType::from);
    let id = value.id().to_string();
    let exec = if let Some(git2::RebaseOperationType::Exec) = value.kind() {
      value.exec().map(|x| x.to_string())
    } else {
      None
    };
    Self { kind, id, exec }
  }
}

#[napi(string_enum)]
/// A rebase operation.
/// Describes a single instruction/operation to be performed during the
/// rebase.
///
/// - `Pick` : The given commit is to be cherry-picked. The client should commit the
/// changes and continue if there are no conflicts.
/// - `Reword` : The given commit is to be cherry-picked, but the client should prompt
/// the user to provide an updated commit message.
/// - `Edit` : The given commit is to be cherry-picked, but the client should stop to
/// allow the user to edit the changes before committing them.
/// - `Squash` : The given commit is to be squashed into the previous commit. The commit
/// message will be merged with the previous message.
/// - `Fixup` : The given commit is to be squashed into the previous commit. The commit
/// message from this commit will be discarded.
/// - `Exec` : No commit will be cherry-picked. The client should run the given command
/// and (if successful) continue.
pub enum RebaseOperationType {
  Pick,
  Reword,
  Edit,
  Squash,
  Fixup,
  Exec,
}

impl From<RebaseOperationType> for git2::RebaseOperationType {
  fn from(value: RebaseOperationType) -> Self {
    match value {
      RebaseOperationType::Pick => git2::RebaseOperationType::Pick,
      RebaseOperationType::Reword => git2::RebaseOperationType::Reword,
      RebaseOperationType::Edit => git2::RebaseOperationType::Edit,
      RebaseOperationType::Squash => git2::RebaseOperationType::Squash,
      RebaseOperationType::Fixup => git2::RebaseOperationType::Fixup,
      RebaseOperationType::Exec => git2::RebaseOperationType::Exec,
    }
  }
}

impl From<git2::RebaseOperationType> for RebaseOperationType {
  fn from(value: git2::RebaseOperationType) -> Self {
    match value {
      git2::RebaseOperationType::Pick => RebaseOperationType::Pick,
      git2::RebaseOperationType::Reword => RebaseOperationType::Reword,
      git2::RebaseOperationType::Edit => RebaseOperationType::Edit,
      git2::RebaseOperationType::Squash => RebaseOperationType::Squash,
      git2::RebaseOperationType::Fixup => RebaseOperationType::Fixup,
      git2::RebaseOperationType::Exec => RebaseOperationType::Exec,
    }
  }
}

#[napi(object)]
pub struct RebaseOptions {
  /// This will instruct other clients working on this
  /// rebase that you want a quiet rebase experience, which they may choose to
  /// provide in an application-specific manner. This has no effect upon
  /// libgit2 directly, but is provided for interoperability between Git
  /// tools.
  pub quiet: Option<bool>,
  /// This will begin an in-memory rebase,
  /// which will allow callers to step through the rebase operations and
  /// commit the rebased changes, but will not rewind HEAD or update the
  /// repository to be in a rebasing state.  This will not interfere with
  /// the working directory (if there is one).
  pub inmemory: Option<bool>,
  /// Used by `finish()`, this is the name of the notes reference
  /// used to rewrite notes for rebased commits when finishing the rebase;
  /// if NULL, the contents of the configuration option `notes.rewriteRef`
  /// is examined, unless the configuration option `notes.rewrite.rebase`
  /// is set to false.
  /// If `notes.rewriteRef` is also NULL, notes will not be rewritten.
  pub rewrite_notes_ref: Option<String>,
  /// Options to control how trees are merged during `next()`.
  pub merge_options: Option<MergeOptions>,
  /// Options to control how files are written during `Repository::rebase`,
  /// `next()` and `abort()`. Note that a minimum strategy of
  /// `GIT_CHECKOUT_SAFE` is defaulted in `init` and `next`, and a minimum
  /// strategy of `GIT_CHECKOUT_FORCE` is defaulted in `abort` to match git
  /// semantics.
  pub checkout_options: Option<CheckoutOptions>,
}

impl From<RebaseOptions> for git2::RebaseOptions<'_> {
  fn from(value: RebaseOptions) -> Self {
    let mut builder = git2::RebaseOptions::new();
    if let Some(quiet) = value.quiet {
      builder.quiet(quiet);
    }
    if let Some(inmemory) = value.inmemory {
      builder.inmemory(inmemory);
    }
    if let Some(rewrite_notes_ref) = value.rewrite_notes_ref {
      builder.rewrite_notes_ref(&rewrite_notes_ref);
    }
    if let Some(merge_options) = value.merge_options {
      builder.merge_options(git2::MergeOptions::from(merge_options));
    }
    if let Some(checkout_options) = value.checkout_options {
      builder.checkout_options(git2::build::CheckoutBuilder::from(checkout_options));
    }
    builder
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Initializes a rebase operation to rebase the changes in `branch`
  /// relative to `upstream` onto another branch. To begin the rebase process,
  /// call iterator.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   rebase(
  ///     branch?: AnnotatedCommit | undefined | null,
  ///     upstream?: AnnotatedCommit | undefined | null,
  ///     onto?: AnnotatedCommit | undefined | null,
  ///     options?: RebaseOptions | undefined | null,
  ///   ): Rebase;
  /// }
  /// ```
  ///
  /// @param {AnnotatedCommit | undefined | null} [branch] - Annotated commit representing the
  /// branch to rebase. Typically, the branch's head commit. If omitted, the currently checked-out
  /// branch is used.
  /// @param {AnnotatedCommit | undefined | null} [upstream] - Annotated commit that defines the
  /// "original base" of the commits to be rebased. If omitted, the repository will typically try
  /// to use the branch's configured upstream.
  /// @param {AnnotatedCommit | undefined | null} [onto] - Specified the "new base" onto which the
  /// selected commits will be reapplied.
  /// @param {RebaseOptions | undefined | null} [options] - Fine-grained control of the rebase
  /// behavior, such as checkout options, merge options, and in-memory rebase.
  /// @returns The initialized rebase handle to iterate and apply steps.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const branchRef = repo.getReference('refs/heads/other');
  /// const upstreamRef = repo.getReference('refs/heads/main');
  /// const branch = repo.getAnnotatedCommitFromReference(branchRef);
  /// const upstream = repo.getAnnotatedCommitFromReference(upstreamRef);
  ///
  /// const sig = { name: 'Seokju Na', email: 'seokju.me@toss.im' };
  ///
  /// const rebase = repo.rebase(branch, upstream);
  /// for (const op of rebase) {
  ///   rebase.commit({ committer: sig });
  /// }
  /// rebase.finish(sig);
  /// ```
  pub fn rebase(
    &self,
    branch: Option<&AnnotatedCommit>,
    upstream: Option<&AnnotatedCommit>,
    onto: Option<&AnnotatedCommit>,
    options: Option<RebaseOptions>,
    env: Env,
    this: Reference<Repository>,
  ) -> crate::Result<Rebase> {
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .rebase(
          branch.map(|x| x.deref()),
          upstream.map(|x| x.deref()),
          onto.map(|x| x.deref()),
          options.map(git2::RebaseOptions::from).as_mut(),
        )
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Rebase { inner })
  }

  #[napi]
  /// Opens an existing rebase that was previously started by either an
  /// invocation of `rebase()` or by another client.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   openRebase(options?: RebaseOptions | undefined | null): Rebase;
  /// }
  /// ```
  ///
  /// @param {RebaseOptions | undefined | null} [options] - Fine-grained control of the rebase
  /// behavior, such as checkout options, merge options, and in-memory rebase.
  /// @returns The initialized rebase handle to iterate and apply steps.
  /// @throws Throws if the existing rebase was not found.
  pub fn open_rebase(
    &self,
    options: Option<RebaseOptions>,
    env: Env,
    this: Reference<Repository>,
  ) -> crate::Result<Rebase> {
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .open_rebase(options.map(git2::RebaseOptions::from).as_mut())
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Rebase { inner })
  }
}
