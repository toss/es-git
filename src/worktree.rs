use crate::repository::Repository;
use crate::util::path_to_string;
use napi::bindgen_prelude::{Reference, SharedReference};
use napi::Env;
use napi_derive::napi;
use std::ops::Deref;
use std::path::Path;

#[napi(string_enum)]
/// Lock Status of a worktree
pub enum WorktreeLockStatus {
  /// Worktree is Unlocked
  Unlocked,
  /// Worktree is locked with the optional message
  Locked(Option<String>),
}

#[napi(object)]
/// Options for adding a worktree.
pub struct WorktreeAddOptions {
  /// If enabled, this will cause the newly added worktree to be locked.
  /// Defaults to `false`.
  pub lock: Option<bool>,

  /// If enabled, this will checkout the existing branch matching the worktree name.
  /// Defaults to `false`.
  pub checkout_existing: Option<bool>,

  /// reference name to use for the new worktree HEAD
  /// Defaults to `null`.
  pub ref_name: Option<String>,
}

#[napi(object)]
/// Options to configure how worktree pruning is performed.
pub struct WorktreePruneOptions {
  /// Controls whether valid (still existing on the filesystem) worktrees will be pruned.
  /// Defaults to `false`.
  pub valid: Option<bool>,

  /// Controls whether locked worktrees will be pruned.
  /// Defaults to `false`.
  pub locked: Option<bool>,

  /// Controls whether the actual working tree on the filesystem is recursively removed.
  /// Defaults to `false`.
  pub working_tree: Option<bool>,
}

impl From<WorktreePruneOptions> for git2::WorktreePruneOptions {
  fn from(value: WorktreePruneOptions) -> Self {
    let mut git2_worktree_prune_options = git2::WorktreePruneOptions::new();
    value.valid.map(|_valid| git2_worktree_prune_options.valid(_valid));
    value.valid.map(|_locked| git2_worktree_prune_options.locked(_locked));
    value
      .valid
      .map(|_working_tree| git2_worktree_prune_options.working_tree(_working_tree));
    git2_worktree_prune_options
  }
}

impl From<git2::WorktreeLockStatus> for WorktreeLockStatus {
  fn from(value: git2::WorktreeLockStatus) -> Self {
    match value {
      git2::WorktreeLockStatus::Unlocked => WorktreeLockStatus::Unlocked,
      git2::WorktreeLockStatus::Locked(reason) => WorktreeLockStatus::Locked(reason),
    }
  }
}

pub(crate) enum WorktreeInner {
  Repo(SharedReference<Repository, git2::Worktree>),
}

impl Deref for WorktreeInner {
  type Target = git2::Worktree;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(inner) => inner.deref(),
    }
  }
}

#[napi]
/// A class to represent a git worktree.
pub struct Worktree {
  pub(crate) inner: WorktreeInner,
}

#[napi]
impl Worktree {
  #[napi]
  /// Open a worktree from a repository.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   static openFromRepository(repo: Repository): Worktree;
  /// }
  /// ```
  ///
  /// @param {Repository} repo - Repository to open worktree from.
  /// @returns Worktree instance.
  /// @throws Throws error if the repository is not a worktree or if opening fails.
  pub fn open_from_repository(repo: Reference<Repository>, env: Env) -> crate::Result<Worktree> {
    let worktree = repo.share_with(env, |repo| {
      git2::Worktree::open_from_repository(&repo.inner)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Worktree {
      inner: WorktreeInner::Repo(worktree),
    })
  }

  #[napi]
  /// Get the name of this worktree.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   name(): string | null;
  /// }
  /// ```
  ///
  /// @returns Name of this worktree. Returns `null` if the worktree has no name.
  pub fn name(&self) -> Option<String> {
    self.inner.name().map(String::from)
  }

  #[napi]
  /// Get the path of this worktree.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   path(): string;
  /// }
  /// ```
  ///
  /// @returns Path of this worktree.
  pub fn path(&self) -> String {
    path_to_string(self.inner.path())
  }

  #[napi]
  /// Validate that the worktree is in a valid state.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   validate(): void;
  /// }
  /// ```
  ///
  /// @throws Throws error if the worktree is in an invalid state.
  pub fn validate(&self) -> crate::Result<()> {
    self.inner.validate().map_err(crate::Error::from)
  }

  #[napi]
  /// Lock the worktree.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   lock(reason?: string | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {string} [reason] - Optional reason for locking the worktree.
  /// @throws Throws error if locking fails.
  pub fn lock(&self, reason: Option<String>) -> crate::Result<()> {
    self.inner.lock(reason.as_deref()).map_err(crate::Error::from)
  }

  #[napi]
  /// Unlock the worktree.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   unlock(): void;
  /// }
  /// ```
  ///
  /// @throws Throws error if unlocking fails.
  pub fn unlock(&self) -> crate::Result<()> {
    self.inner.unlock().map_err(crate::Error::from)
  }

  #[napi]
  /// Check if the worktree is locked.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   isLocked(): WorktreeLockStatus;
  /// }
  /// ```
  ///
  /// @returns Lock status of the worktree.
  /// @throws Throws error if checking the lock status fails.
  pub fn is_locked(&self) -> crate::Result<WorktreeLockStatus> {
    let git2_lock_status = self.inner.is_locked()?;
    Ok(git2_lock_status.into())
  }

  #[napi]
  /// Prune the worktree.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   prune(options?: WorktreePruneOptions | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {WorktreePruneOptions} [options] - Options for pruning the worktree.
  /// @throws Throws error if pruning fails.
  pub fn prune(&self, worktree_prune_options: Option<WorktreePruneOptions>) -> crate::Result<()> {
    let mut git2_worktree_prune_options = worktree_prune_options.map(git2::WorktreePruneOptions::from);
    self
      .inner
      .prune(git2_worktree_prune_options.as_mut())
      .map_err(crate::Error::from)
  }

  #[napi]
  /// Check if the worktree is prunable.
  ///
  /// @category Worktree/Methods
  ///
  /// @signature
  /// ```ts
  /// class Worktree {
  ///   isPrunable(options?: WorktreePruneOptions | null | undefined): boolean;
  /// }
  /// ```
  ///
  /// @param {WorktreePruneOptions} [options] - Options for checking if the worktree is prunable.
  /// @returns `true` if the worktree is prunable, `false` otherwise.
  /// @throws Throws error if checking fails.
  pub fn is_prunable(&self, worktree_prune_options: Option<WorktreePruneOptions>) -> crate::Result<bool> {
    let mut git2_worktree_prune_options = worktree_prune_options.map(git2::WorktreePruneOptions::from);
    self
      .inner
      .is_prunable(git2_worktree_prune_options.as_mut())
      .map_err(crate::Error::from)
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Add a new worktree to the repository.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   worktree(name: string, path: string, options?: WorktreeAddOptions | null | undefined): Worktree;
  /// }
  /// ```
  ///
  /// @param {string} name - Name of the worktree to add.
  /// @param {string} path - Path where the worktree should be created.
  /// @param {WorktreeAddOptions} [options] - Options for adding the worktree.
  /// @returns New worktree instance.
  /// @throws Throws error if adding the worktree fails (e.g., path already exists, invalid reference name, or filesystem errors).
  pub fn worktree(
    &self,
    this: Reference<Repository>,
    env: Env,
    name: String,
    path: String,
    options: Option<WorktreeAddOptions>,
  ) -> crate::Result<Worktree> {
    let mut git2_opts = git2::WorktreeAddOptions::new();
    // add non reference options
    if let Some(ref _options) = options {
      _options.lock.map(|_lock| git2_opts.lock(_lock));
      _options
        .checkout_existing
        .map(|_checkout_existing| git2_opts.checkout_existing(_checkout_existing));
    }

    // add reference option
    let git2_reference = options
      .as_ref()
      .and_then(|opts| opts.ref_name.as_ref())
      .map(|ref_name| self.inner.find_reference(ref_name))
      .transpose()?;
    git2_opts.reference(git2_reference.as_ref());

    // add worktree
    let git2_worktree = this.share_with(env, |repo| {
      repo
        .inner
        .worktree(&name, Path::new(&path), Some(&git2_opts))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;

    Ok(Worktree {
      inner: WorktreeInner::Repo(git2_worktree),
    })
  }

  #[napi]
  /// List all worktrees in the repository.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   worktrees(): string[];
  /// }
  /// ```
  ///
  /// @returns Array of worktree names.
  /// @throws Throws error if listing worktrees fails (e.g., filesystem errors or repository corruption).
  pub fn worktrees(&self) -> crate::Result<Vec<String>> {
    let git2_worktrees = self.inner.worktrees()?;
    let worktree_names: Vec<String> = git2_worktrees
      .iter()
      .filter_map(|name| name.map(ToString::to_string))
      .collect::<Vec<String>>();
    Ok(worktree_names)
  }

  #[napi]
  /// Tests whether this repository is a worktree.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   isWorktree(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if repository is a worktree.
  pub fn is_worktree(&self) -> bool {
    self.inner.is_worktree()
  }

  #[napi]
  /// Find a worktree by name.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findWorktree(name: string): Worktree;
  /// }
  /// ```
  ///
  /// @param {string} name - Name of the worktree to find.
  /// @returns Worktree instance.
  /// @throws Throws error if the worktree is not found or if opening fails.
  pub fn find_worktree(&self, this: Reference<Repository>, env: Env, name: String) -> crate::Result<Worktree> {
    let git2_worktree = this.share_with(env, |repo| {
      repo
        .inner
        .find_worktree(&name)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Worktree {
      inner: WorktreeInner::Repo(git2_worktree),
    })
  }

  #[napi]
  /// Open a repository from a worktree.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   static openFromWorktree(worktree: Worktree): Repository;
  /// }
  /// ```
  ///
  /// @param {Worktree} worktree - Worktree to open repository from.
  /// @returns Repository instance.
  /// @throws Throws error if opening the repository fails.
  pub fn open_from_worktree(worktree: &Worktree) -> crate::Result<Repository> {
    let git2_repository = git2::Repository::open_from_worktree(&worktree.inner)?;
    Ok(Repository { inner: git2_repository })
  }
}
