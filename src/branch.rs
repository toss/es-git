use crate::commit::Commit;
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::{Deref, DerefMut};

#[napi]
/// Ensure the branch name is well-formed.
///
/// @category Branch
/// @signature
/// ```ts
/// function isValidBranchName(name: string): boolean;
/// ```
///
/// @param {string} name - Branch name to check is valid.
/// @returns Returns `true` if the given branch name is well-formed.
pub fn is_valid_branch_name(name: String) -> crate::Result<bool> {
  Ok(git2::Branch::name_is_valid(&name)?)
}

pub(crate) enum BranchInner {
  Repo(SharedReference<Repository, git2::Branch<'static>>),
  Owned(git2::Branch<'static>),
}

impl Deref for BranchInner {
  type Target = git2::Branch<'static>;

  fn deref(&self) -> &Self::Target {
    match self {
      Self::Repo(x) => x.deref(),
      Self::Owned(x) => x,
    }
  }
}

impl DerefMut for BranchInner {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Self::Repo(x) => x.deref_mut(),
      Self::Owned(x) => x,
    }
  }
}

#[napi]
/// A structure to represent a git [branch][1]
///
/// A branch is currently just a wrapper to an underlying `Reference`. The
/// reference can be accessed through the `get` and `into_reference` methods.
///
/// [1]: http://git-scm.com/book/en/Git-Branching-What-a-Branch-Is
pub struct Branch {
  pub(crate) inner: BranchInner,
}

#[napi(iterator)]
/// An iterator over the branches inside of a repository.
pub struct Branches {
  pub(crate) inner: SharedReference<Repository, git2::Branches<'static>>,
}

#[napi(object)]
pub struct BranchesItem {
  #[napi(js_name = "type")]
  pub kind: BranchType,
  pub name: String,
}

#[napi]
impl Generator for Branches {
  type Yield = BranchesItem;
  type Next = ();
  type Return = ();

  fn next(&mut self, _value: Option<Self::Next>) -> Option<Self::Yield> {
    self.inner.next().and_then(|x| {
      x.ok().and_then(|(branch, branch_type)| {
        let kind = BranchType::from(branch_type);
        branch
          .name_bytes()
          .ok()
          .and_then(|x| std::str::from_utf8(x).ok())
          .map(|x| x.to_string())
          .map(|name| BranchesItem { kind, name })
      })
    })
  }
}

#[napi(string_enum)]
/// - `Local` : A local branch not on a remote.
/// - `Remote` : A branch for a remote.
pub enum BranchType {
  Local,
  Remote,
}

impl From<BranchType> for git2::BranchType {
  fn from(value: BranchType) -> Self {
    match value {
      BranchType::Local => git2::BranchType::Local,
      BranchType::Remote => git2::BranchType::Remote,
    }
  }
}

impl From<git2::BranchType> for BranchType {
  fn from(value: git2::BranchType) -> Self {
    match value {
      git2::BranchType::Local => BranchType::Local,
      git2::BranchType::Remote => BranchType::Remote,
    }
  }
}

#[napi(object)]
pub struct BranchRenameOptions {
  /// If the force flag is not enabled, and there's already a branch with
  /// the given name, the renaming will fail.
  pub force: Option<bool>,
}

#[napi]
impl Branch {
  #[napi]
  /// Get the OID pointed to by a reference which is this branch.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   referenceTarget(): string | null;
  /// }
  /// ```
  ///
  /// @returns The OID pointed to by a reference which is this branch.
  pub fn reference_target(&self) -> Option<String> {
    let reference = self.inner.get();
    reference.target().map(|x| x.to_string())
  }

  #[napi]
  /// Delete an existing branch reference.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   delete(): void;
  /// }
  /// ```
  pub fn delete(&mut self) -> crate::Result<()> {
    self.inner.delete()?;
    Ok(())
  }

  #[napi]
  /// Determine if the current local branch is pointed at by `HEAD`.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   isHead(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if the current local branch is pointed at by `HEAD`.
  pub fn is_head(&self) -> bool {
    self.inner.is_head()
  }

  #[napi]
  /// Move/rename an existing local branch reference.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   rename(newBranchName: string, options?: BranchRenameOptions | null | undefined): Branch;
  /// }
  /// ```
  ///
  /// @param {string} newBranchName - Branch name to move/rename.
  /// @param {BranchRenameOptions} [options] - Options for move/rename branch.
  /// @returns Move/renamed branch.
  pub fn rename(&mut self, new_branch_name: String, options: Option<BranchRenameOptions>) -> crate::Result<Branch> {
    let inner = self
      .inner
      .rename(&new_branch_name, options.and_then(|x| x.force).unwrap_or_default())?;
    Ok(Branch {
      inner: BranchInner::Owned(inner),
    })
  }

  #[napi]
  /// Return the name of the given local or remote branch.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   name(): string;
  /// }
  /// ```
  ///
  /// @returns The name of the given local or remote branch.
  /// @throws If the name is not valid utf-8.
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes()?)?.to_string();
    Ok(name)
  }

  #[napi]
  /// Return the reference supporting the remote tracking branch, given a
  /// local branch reference.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   findUpstream(): Branch | null;
  /// }
  /// ```
  ///
  /// @returns The reference supporting the remote tacking branch.
  pub fn find_upstream(&self) -> Option<Branch> {
    self.get_upstream().ok()
  }

  #[napi]
  /// Return the reference supporting the remote tracking branch, given a
  /// local branch reference.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   getUpstream(): Branch;
  /// }
  /// ```
  ///
  /// @returns The reference supporting the remote tacking branch.
  /// @throws Throws error if upstream does not exist.
  pub fn get_upstream(&self) -> crate::Result<Branch> {
    let inner = self.inner.upstream()?;
    Ok(Branch {
      inner: BranchInner::Owned(inner),
    })
  }

  #[napi]
  /// Set the upstream configuration for a given local branch.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   setUpstream(upstreamName: string): void;
  /// }
  /// ```
  ///
  /// @param {string} upstreamName - Branch name to set as upstream.
  pub fn set_upstream(&mut self, upstream_name: String) -> crate::Result<()> {
    self.inner.set_upstream(Some(&upstream_name))?;
    Ok(())
  }

  #[napi]
  /// Unset the upstream configuration for a given local branch.
  ///
  /// @category Branch/Methods
  /// @signature
  /// ```ts
  /// class Branch {
  ///   unsetUpstream(): void;
  /// }
  /// ```
  pub fn unset_upstream(&mut self) -> crate::Result<()> {
    self.inner.set_upstream(None)?;
    Ok(())
  }
}

#[napi(object)]
pub struct CreateBranchOptions {
  /// If `force` is true and a reference already exists with the given name,
  /// it'll be replaced.
  pub force: Option<bool>,
}

#[napi(object)]
pub struct BranchesFilter {
  #[napi(js_name = "type")]
  /// Branch type to filter.
  pub kind: Option<BranchType>,
}

#[napi]
impl Repository {
  #[napi]
  /// Create a new branch pointing at a target commit
  ///
  /// A new direct reference will be created pointing to this target commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   createBranch(
  ///     branchName: string,
  ///     target: Commit,
  ///     options?: CreateBranchOptions | null | undefined,
  ///   ): Branch;
  /// }
  /// ```
  ///
  /// @param {string} branchName - Name for the new branch.
  /// @param {Commit} target - Target commit which will be pointed by this branch.
  /// @param {CreateBranchOptions} [options] - Options for create branch.
  /// @returns {Branch} Newly created branch.
  pub fn create_branch(
    &self,
    branch_name: String,
    target: &Commit,
    options: Option<CreateBranchOptions>,
    env: Env,
    this: Reference<Repository>,
  ) -> crate::Result<Branch> {
    let force = options.and_then(|x| x.force).unwrap_or_default();
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .branch(&branch_name, &target.inner, force)
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Branch {
      inner: BranchInner::Repo(inner),
    })
  }

  #[napi]
  /// Lookup a branch by its name in a repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findBranch(name: string, branchType: BranchType): Branch | null;
  /// }
  /// ```
  ///
  /// @param {string} name - A branch name.
  /// @param {BranchType} branchType - Branch type to lookup.
  /// @returns A found branch.
  pub fn find_branch(
    &self,
    name: String,
    branch_type: BranchType,
    env: Env,
    this: Reference<Repository>,
  ) -> Option<Branch> {
    self.get_branch(name, branch_type, env, this).ok()
  }

  #[napi]
  /// Lookup a branch by its name in a repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getBranch(name: string, branchType: BranchType): Branch;
  /// }
  /// ```
  ///
  /// @param {string} name - A branch name.
  /// @param {BranchType} branchType - Branch type to lookup.
  /// @returns A found branch.
  /// @throws Throws error if branch does not exist.
  pub fn get_branch(
    &self,
    name: String,
    branch_type: BranchType,
    env: Env,
    this: Reference<Repository>,
  ) -> crate::Result<Branch> {
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .find_branch(&name, branch_type.into())
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Branch {
      inner: BranchInner::Repo(inner),
    })
  }

  #[napi]
  /// Create an iterator which loops over the requested branches.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   branches(filter?: BranchesFilter | null | undefined): Branches;
  /// }
  /// ```
  ///
  /// @param {BranchesFilter} [filter] - Filter for the branches iterator.
  /// @returns An iterator which loops over the requested branches.
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  ///
  /// for (const branch of repo.branches()) {
  ///   console.log(branch.type); // "Local"
  ///   console.log(branch.name); // "main"
  /// }
  /// ```
  pub fn branches(
    &self,
    filter: Option<BranchesFilter>,
    env: Env,
    this: Reference<Repository>,
  ) -> crate::Result<Branches> {
    let inner = this.share_with(env, move |repo| {
      repo
        .inner
        .branches(filter.and_then(|x| x.kind.map(|x| x.into())))
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
    })?;
    Ok(Branches { inner })
  }
}
