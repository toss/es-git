use crate::checkout::CheckoutOptions;
use crate::remote::FetchOptions;
use crate::repository::Repository;
use crate::util;
use crate::util::bitflags_contain;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::ops::DerefMut;
use std::path::Path;

#[napi(string_enum)]
/// Submodule ignore values
///
/// These values represent settings for the `submodule.$name.ignore`
/// configuration value which says how deeply to look at the working
/// directory when getting the submodule status.
pub enum SubmoduleIgnore {
  /// Use the submodule's configuration
  Unspecified,
  /// Any change or untracked file is considered dirty
  None,
  /// Only dirty if tracked files have changed
  Untracked,
  /// Only dirty if HEAD has moved
  Dirty,
  /// Never dirty
  All,
}

impl From<SubmoduleIgnore> for git2::SubmoduleIgnore {
  fn from(value: SubmoduleIgnore) -> Self {
    match value {
      SubmoduleIgnore::Unspecified => git2::SubmoduleIgnore::Unspecified,
      SubmoduleIgnore::None => git2::SubmoduleIgnore::None,
      SubmoduleIgnore::Untracked => git2::SubmoduleIgnore::Untracked,
      SubmoduleIgnore::Dirty => git2::SubmoduleIgnore::Dirty,
      SubmoduleIgnore::All => git2::SubmoduleIgnore::All,
    }
  }
}

impl From<git2::SubmoduleIgnore> for SubmoduleIgnore {
  fn from(value: git2::SubmoduleIgnore) -> Self {
    match value {
      git2::SubmoduleIgnore::Unspecified => SubmoduleIgnore::Unspecified,
      git2::SubmoduleIgnore::None => SubmoduleIgnore::None,
      git2::SubmoduleIgnore::Untracked => SubmoduleIgnore::Untracked,
      git2::SubmoduleIgnore::Dirty => SubmoduleIgnore::Dirty,
      git2::SubmoduleIgnore::All => SubmoduleIgnore::All,
    }
  }
}

/// Submodule update values
///
/// These values represent settings for the `submodule.$name.update`
/// configuration value which says how to handle `git submodule update`
/// for this submodule. The value is usually set in the ".gitmodules"
/// file and copied to ".git/config" when the submodule is initialized.
#[napi(string_enum)]
pub enum SubmoduleUpdate {
  /// The default; when a submodule is updated, checkout the new detached
  /// HEAD to the submodule directory.
  Checkout,
  /// Update by rebasing the current checked out branch onto the commit from
  /// the superproject.
  Rebase,
  /// Update by merging the commit in the superproject into the current
  /// checkout out branch of the submodule.
  Merge,
  /// Do not update this submodule even when the commit in the superproject
  /// is updated.
  None,
  /// Not used except as static initializer when we don't want any particular
  /// update rule to be specified.
  Default,
}

impl From<SubmoduleUpdate> for git2::SubmoduleUpdate {
  fn from(value: SubmoduleUpdate) -> Self {
    match value {
      SubmoduleUpdate::Checkout => git2::SubmoduleUpdate::Checkout,
      SubmoduleUpdate::Rebase => git2::SubmoduleUpdate::Rebase,
      SubmoduleUpdate::Merge => git2::SubmoduleUpdate::Merge,
      SubmoduleUpdate::None => git2::SubmoduleUpdate::None,
      SubmoduleUpdate::Default => git2::SubmoduleUpdate::Default,
    }
  }
}

impl From<git2::SubmoduleUpdate> for SubmoduleUpdate {
  fn from(value: git2::SubmoduleUpdate) -> Self {
    match value {
      git2::SubmoduleUpdate::Checkout => SubmoduleUpdate::Checkout,
      git2::SubmoduleUpdate::Rebase => SubmoduleUpdate::Rebase,
      git2::SubmoduleUpdate::Merge => SubmoduleUpdate::Merge,
      git2::SubmoduleUpdate::None => SubmoduleUpdate::None,
      git2::SubmoduleUpdate::Default => SubmoduleUpdate::Default,
    }
  }
}

#[napi]
#[repr(u32)]
/// Return codes for submodule status.
///
/// A combination of these flags will be returned to describe the status of a
/// submodule.  Depending on the "ignore" property of the submodule, some of
/// the flags may never be returned because they indicate changes that are
/// supposed to be ignored.
///
/// Submodule info is contained in 4 places: the HEAD tree, the index, config
/// files (both .git/config and .gitmodules), and the working directory.  Any
/// or all of those places might be missing information about the submodule
/// depending on what state the repo is in.  We consider all four places to
/// build the combination of status flags.
///
/// There are four values that are not really status, but give basic info
/// about what sources of submodule data are available.  These will be
/// returned even if ignore is set to "ALL".
///
/// * IN_HEAD   - superproject head contains submodule
/// * IN_INDEX  - superproject index contains submodule
/// * IN_CONFIG - superproject gitmodules has submodule
/// * IN_WD     - superproject workdir has submodule
///
/// The following values will be returned so long as ignore is not "ALL".
///
/// * INDEX_ADDED       - in index, not in head
/// * INDEX_DELETED     - in head, not in index
/// * INDEX_MODIFIED    - index and head don't match
/// * WD_UNINITIALIZED  - workdir contains empty directory
/// * WD_ADDED          - in workdir, not index
/// * WD_DELETED        - in index, not workdir
/// * WD_MODIFIED       - index and workdir head don't match
///
/// The following can only be returned if ignore is "NONE" or "UNTRACKED".
///
/// * WD_INDEX_MODIFIED - submodule workdir index is dirty
/// * WD_WD_MODIFIED    - submodule workdir has modified files
///
/// Lastly, the following will only be returned for ignore "NONE".
///
/// * WD_UNTRACKED      - workdir contains untracked files
pub enum SubmoduleStatus {
  InHead = 1,
  InIndex = 2,
  InConfig = 4,
  InWD = 8,
  IndexAdded = 16,
  IndexDeleted = 32,
  IndexModified = 64,
  WDUninitialized = 128,
  WDAdded = 256,
  WDDeleted = 512,
  WDModified = 1024,
  WDIndexModified = 2048,
  WDWDModified = 4096,
  WDUntracked = 8192,
}

#[napi]
/// Check submodule status contains given value.
///
/// @category Submodule
/// @signature
/// ```ts
/// function submoduleStatusContains(source: number, target: number): boolean;
/// ```
///
/// @param {number} source - Source status.
/// @param {number} target - Target status.
/// @returns Returns `true` is source status contains target status.
pub fn submodule_status_contains(source: u32, target: u32) -> bool {
  bitflags_contain(
    git2::SubmoduleStatus::from_bits_retain(source),
    git2::SubmoduleStatus::from_bits_retain(target),
  )
}

#[napi]
pub struct Submodule {
  pub(crate) inner: SharedReference<Repository, git2::Submodule<'static>>,
}

pub struct SubmoduleInitTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
  overwrite: bool,
}

unsafe impl Send for SubmoduleInitTask {}

#[napi]
impl Task for SubmoduleInitTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    self.inner.init(self.overwrite).map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct SubmoduleRepoInitTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
  use_gitlink: bool,
}

unsafe impl Send for SubmoduleRepoInitTask {}

#[napi]
impl Task for SubmoduleRepoInitTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let repo = self.inner.repo_init(self.use_gitlink).map_err(crate::Error::from)?;
    Ok(Repository { inner: repo })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

pub struct SubmoduleOpenTask {
  pub(crate) inner: SharedReference<Repository, git2::Submodule<'static>>,
}

unsafe impl Send for SubmoduleOpenTask {}

#[napi]
impl Task for SubmoduleOpenTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let repo = self.inner.open().map_err(crate::Error::from)?;
    Ok(Repository { inner: repo })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

pub struct SubmoduleSyncTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
}

unsafe impl Send for SubmoduleSyncTask {}

#[napi]
impl Task for SubmoduleSyncTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    self.inner.sync().map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct SubmoduleReloadTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
  force: bool,
}

unsafe impl Send for SubmoduleReloadTask {}

#[napi]
impl Task for SubmoduleReloadTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    self.inner.reload(self.force).map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(object, object_to_js = false)]
/// Options to update a submodule.
pub struct SubmoduleUpdateOptions {
  /// These options are passed to the checkout step.
  pub checkout: Option<CheckoutOptions>,
  /// Options which control the fetch, including callbacks.
  pub fetch: Option<FetchOptions>,
  /// Allow fetching from the submodule's default remote if the target commit isn't found.
  /// Default: `true`.
  pub allow_fetch: Option<bool>,
}

pub struct SubmoduleUpdateTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
  init: bool,
  options: Option<SubmoduleUpdateOptions>,
}

unsafe impl Send for SubmoduleUpdateTask {}

#[napi]
impl Task for SubmoduleUpdateTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let mut opts = git2::SubmoduleUpdateOptions::default();
    if let Some(options) = &self.options {
      if let Some(checkout) = &options.checkout {
        opts.checkout(checkout.clone().into());
      }
      if let Some(fetch) = &options.fetch {
        opts.fetch(fetch.to_git2_fetch_options());
      }
      if let Some(allow_fetch) = options.allow_fetch {
        opts.allow_fetch(allow_fetch);
      }
    }
    self
      .inner
      .update(self.init, Some(&mut opts))
      .map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct SubmoduleCloneTask {
  inner: SharedReference<Repository, git2::Submodule<'static>>,
  options: Option<SubmoduleUpdateOptions>,
}

unsafe impl Send for SubmoduleCloneTask {}

#[napi]
impl Task for SubmoduleCloneTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut opts = git2::SubmoduleUpdateOptions::default();
    if let Some(options) = &self.options {
      if let Some(checkout) = &options.checkout {
        opts.checkout(checkout.clone().into());
      }
      if let Some(fetch) = &options.fetch {
        opts.fetch(fetch.to_git2_fetch_options());
      }
      if let Some(allow_fetch) = options.allow_fetch {
        opts.allow_fetch(allow_fetch);
      }
    }
    let inner = self
      .inner
      .deref_mut()
      .clone(Some(&mut opts))
      .map_err(crate::Error::from)?;
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
impl Submodule {
  #[napi]
  /// Get the name for the submodule.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   name(): string;
  /// }
  /// ```
  ///
  pub fn name(&self) -> crate::Result<String> {
    let name = std::str::from_utf8(self.inner.name_bytes())?.to_string();
    Ok(name)
  }

  #[napi]
  /// Get the submodule's branch.
  ///ule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   branch(): string | null;
  /// }
  /// ```
  /// @returns The branch name of the submodule. Returns `null` if the branch if the branch is
  /// not yet available.
  pub fn branch(&self) -> crate::Result<Option<String>> {
    match self.inner.branch_bytes() {
      Some(bytes) => {
        let branch = std::str::from_utf8(bytes)?.to_string();
        Ok(Some(branch))
      }
      None => Ok(None),
    }
  }

  #[napi]
  /// Get the submodule's URL.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   url(): string | null;
  /// }
  /// ```
  ///
  /// @returns The URL of the submodule. Returns `null` if the URL isn't present.
  pub fn url(&self) -> crate::Result<Option<String>> {
    match self.inner.opt_url_bytes() {
      Some(bytes) => {
        let url = std::str::from_utf8(bytes)?.to_string();
        Ok(Some(url))
      }
      None => Ok(None),
    }
  }

  #[napi]
  /// Get the path for the submodule.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   path(): string;
  /// }
  /// ```
  ///
  /// @returns The path for the submodule.
  pub fn path(&self) -> String {
    util::path_to_string(self.inner.path())
  }

  #[napi]
  /// Get the OID for the submodule in the current `HEAD` tree.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   headId(): string | null;
  /// }
  /// ```
  ///
  /// @returns The OID for the submodule in the current `HEAD` tree.
  pub fn head_id(&self) -> Option<String> {
    self.inner.head_id().map(|x| x.to_string())
  }

  #[napi]
  /// Get the OID for the submodule in the index.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   indexId(): string | null;
  /// }
  /// ```
  ///
  /// @returns The OID for the submodule in the index.
  pub fn index_id(&self) -> Option<String> {
    self.inner.index_id().map(|x| x.to_string())
  }

  #[napi]
  /// Get the OID for the submodule in the current working directory.
  ///
  /// This returns the OID that corresponds to looking up `HEAD` in the
  /// checked out submodule. If there are pending changes in the index or
  /// anything else, this won't notice that.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   workdirId(): string | null;
  /// }
  /// ```
  ///
  /// @returns The OID for the submodule in the current working directory.
  pub fn workdir_id(&self) -> Option<String> {
    self.inner.workdir_id().map(|x| x.to_string())
  }

  #[napi]
  /// Get the ignore rule that will be used for the submodule.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   ignoreRule(): SubmoduleIgnore;
  /// }
  /// ```
  ///
  /// @returns The ignore rule that will be used for the submodule.
  pub fn ignore_rule(&self) -> SubmoduleIgnore {
    self.inner.ignore_rule().into()
  }

  #[napi]
  /// Get the update rule that will be used for the submodule.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   updateStrategy(): SubmoduleUpdate;
  /// }
  /// ```
  ///
  /// @returns The update rule that will be used for the submodule.
  pub fn update_strategy(&self) -> SubmoduleUpdate {
    self.inner.update_strategy().into()
  }

  #[napi]
  /// Copy submodule info into ".git/config" file.
  ///
  /// Just like "git submodule init", this copies information about the
  /// submodule into ".git/config". You can use the accessor functions above
  /// to alter the in-memory git_submodule object and control what is written
  /// to the config, overriding what is in .gitmodules.
  ///
  /// By default, existing entries will not be overwritten, but passing `true`
  /// for `overwrite` forces them to be updated.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   init(
  ///     overwrite?: boolean | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {boolean} [overwrite] - By default, existing entries will not be overwritten, but
  /// setting this to true forces them to be updated.
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  pub fn init(
    &mut self,
    env: Env,
    overwrite: Option<bool>,
    signal: Option<AbortSignal>,
  ) -> crate::Result<AsyncTask<SubmoduleInitTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(
      SubmoduleInitTask {
        inner,
        overwrite: overwrite.unwrap_or_default(),
      },
      signal,
    ))
  }

  #[napi]
  /// Set up the subrepository for a submodule in preparation for clone.
  ///
  /// This function can be called to init and set up a submodule repository
  /// from a submodule in preparation to clone it from its remote.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   repoInit(
  ///     useGitlink?: boolean | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<Repository>;
  /// }
  /// ```
  ///
  /// @param {boolean} [useGitlink] - Should the workdir contain a gitlink to the repo in
  /// `.git/modules` vs. repo directly in workdir.
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  ///
  /// @returns The repository.
  pub fn repo_init(
    &mut self,
    env: Env,
    use_gitlink: Option<bool>,
    signal: Option<AbortSignal>,
  ) -> crate::Result<AsyncTask<SubmoduleRepoInitTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(
      SubmoduleRepoInitTask {
        inner,
        use_gitlink: use_gitlink.unwrap_or_default(),
      },
      signal,
    ))
  }

  #[napi]
  /// Open the repository for a submodule.
  ///
  /// This will only work if the submodule is checked out into the working
  /// directory.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   open(signal?: AbortSignal | null | undefined): Promise<Repository>;
  /// }
  /// ```
  ///
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  /// @returns The repository.
  pub fn open(&self, env: Env, signal: Option<AbortSignal>) -> crate::Result<AsyncTask<SubmoduleOpenTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(SubmoduleOpenTask { inner }, signal))
  }

  #[napi]
  /// Reread submodule info from config, index, and `HEAD`.
  ///
  /// Call this to reread cached submodule information for this submodule if
  /// you have reason to believe that it has changed.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   reload(
  ///     force?: boolean | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {boolean} [force] - If this is `true`, then data will be reloaded even if it
  /// doesn't seem out of date.
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  pub fn reload(
    &self,
    env: Env,
    force: Option<bool>,
    signal: Option<AbortSignal>,
  ) -> crate::Result<AsyncTask<SubmoduleReloadTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(
      SubmoduleReloadTask {
        inner,
        force: force.unwrap_or_default(),
      },
      signal,
    ))
  }

  #[napi]
  /// Copy submodule remote info into submodule repo.
  ///
  /// This copies the information about the submodules URL into the checked
  /// out submodule config, acting like "git submodule sync". This is useful
  /// if you have altered the URL for the submodule (or it has been altered
  /// by a fetch of upstream changes) and you need to update your local repo.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   sync(signal?: AbortSignal | null | undefined): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  pub fn sync(&self, env: Env, signal: Option<AbortSignal>) -> crate::Result<AsyncTask<SubmoduleSyncTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(SubmoduleSyncTask { inner }, signal))
  }

  #[napi]
  /// Add current submodule HEAD commit to index of superproject.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   addToIndex(writeIndex?: boolean | null | undefined): void;
  /// }
  /// ```
  ///
  /// @param {boolean} [writeIndex] - If is true, then the index file will be immediately written.
  /// Otherwise, you must explicitly call `write()` on an `Index` later on.
  pub fn add_to_index(&mut self, write_index: Option<bool>) -> crate::Result<()> {
    self.inner.add_to_index(write_index.unwrap_or_default())?;
    Ok(())
  }

  #[napi]
  /// Resolve the setup of a new git submodule.
  ///
  /// This should be called on a submodule once you have called add setup and
  /// done the clone of the submodule. This adds the `.gitmodules` file and the
  /// newly cloned submodule to the index to be ready to be committed (but
  /// doesn't actually do the commit).
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   addFinalize(): void;
  /// }
  /// ```
  pub fn add_finalize(&mut self) -> crate::Result<()> {
    self.inner.add_finalize()?;
    Ok(())
  }

  #[napi]
  /// Update submodule.
  ///
  /// This will clone a missing submodule and check out the subrepository to
  /// the commit specified in the index of the containing repository. If
  /// the submodule repository doesn't contain the target commit, then the
  /// submodule is fetched using the fetch options supplied in options.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   update(
  ///     init?: boolean | null | undefined,
  ///     options?: SubmoduleUpdateOptions | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {boolean} [init] - Indicates if the submodule should be initialized first if it has
  /// not been initialized yet.
  /// @param {SubmoduleUpdateOptions} [options] - Configuration options for the update.
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  pub fn update(
    &self,
    env: Env,
    init: Option<bool>,
    options: Option<SubmoduleUpdateOptions>,
    signal: Option<AbortSignal>,
  ) -> crate::Result<AsyncTask<SubmoduleUpdateTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(
      SubmoduleUpdateTask {
        inner,
        init: init.unwrap_or_default(),
        options,
      },
      signal,
    ))
  }

  #[napi]
  /// Perform the clone step for a newly created submodule.
  ///
  /// This performs the necessary `git clone` to setup a newly-created submodule.
  ///
  /// @category Submodule/Methods
  /// @signature
  /// ```ts
  /// class Submodule {
  ///   clone(
  ///     options?: SubmoduleUpdateOptions | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<Repository>;
  /// }
  /// ```
  ///
  /// @param {SubmoduleUpdateOptions} [options] - The options to use.
  /// @param {AbortSignal} [signal] - Optional AbortSignal to cancel the operation.
  /// @returns The newly created repository object.
  pub fn clone(
    &self,
    env: Env,
    options: Option<SubmoduleUpdateOptions>,
    signal: Option<AbortSignal>,
  ) -> crate::Result<AsyncTask<SubmoduleCloneTask>> {
    let inner = self.inner.clone(env)?;
    Ok(AsyncTask::with_optional_signal(
      SubmoduleCloneTask { inner, options },
      signal,
    ))
  }
}

#[napi]
impl Repository {
  #[napi]
  /// Set up a new git submodule for checkout.
  ///
  /// This does "git submodule add" up to the fetch and checkout of the
  /// submodule contents. It preps a new submodule, creates an entry in
  /// `.gitmodules` and creates an empty initialized repository either at the
  /// given path in the working directory or in `.git/modules` with a gitlink
  /// from the working directory to the new repo.
  ///
  /// To fully emulate "git submodule add" call this function, then `open()`
  /// the submodule repo and perform the clone step as needed. Lastly, call
  /// `addFinalize()` to wrap up adding the new submodule and `.gitmodules`
  /// to the index to be ready to commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submodule(
  ///     url: string,
  ///     path: string,
  ///     useGitlink?: boolean | null | undefined,
  ///   ): Submodule;
  /// }
  /// ```
  ///
  /// @param {string} url - URL for the submodule's remote.
  /// @param {string} path - Path at which the submodule should be created.
  /// @param {boolean} [useGitlink] - Should workdir contain a gitlink to the repo in
  /// `.git/modules` vs. repo directly in workdir.
  ///
  /// @returns The submodule.
  pub fn submodule(
    &self,
    this: Reference<Repository>,
    env: Env,
    url: String,
    path: String,
    use_gitlink: Option<bool>,
  ) -> crate::Result<Submodule> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .submodule(&url, Path::new(&path), use_gitlink.unwrap_or_default())
        .map_err(crate::Error::from)
        .map_err(Into::into)
    })?;
    Ok(Submodule { inner })
  }

  #[napi]
  pub fn submodules(&self, this: Reference<Repository>, env: Env) -> crate::Result<Vec<Submodule>> {
    let mut submodules = vec![];
    for sub in self.inner.submodules()? {
      if let Ok(name) = std::str::from_utf8(sub.name_bytes()) {
        if let Some(submodule) = self.find_submodule(this.clone(env)?, env, name.to_string()) {
          submodules.push(submodule);
        }
      }
    }
    Ok(submodules)
  }

  #[napi]
  /// Lookup submodule information by name or path.
  ///
  /// Given either the submodule name or path (they are usually the same),
  /// this returns a structure describing the submodule.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getSubmodule(name: string): Submodule;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of or path to the submodule; trailing slashes okay.
  /// @returns The submodule.
  /// @throws If the submodule not found.
  pub fn get_submodule(&self, this: Reference<Repository>, env: Env, name: String) -> crate::Result<Submodule> {
    let inner = this.share_with(env, |repo| {
      repo
        .inner
        .find_submodule(&name)
        .map_err(crate::Error::from)
        .map_err(Into::into)
    })?;
    Ok(Submodule { inner })
  }

  #[napi]
  /// Lookup submodule information by name or path.
  ///
  /// Given either the submodule name or path (they are usually the same),
  /// this returns a structure describing the submodule.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findSubmodule(name: string): Submodule | null;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of or path to the submodule; trailing slashes okay.
  /// @returns The submodule. Returns `null` if the submodule is not found.
  pub fn find_submodule(&self, this: Reference<Repository>, env: Env, name: String) -> Option<Submodule> {
    self.get_submodule(this, env, name).ok()
  }

  #[napi]
  /// Get the status for a submodule.
  ///
  /// This looks at a submodule and tries to determine the status.  It
  /// will return a combination of the `SubmoduleStatus` values.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submoduleStatus(name: string, ignore: SubmoduleIgnore): number;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the submodule.
  /// @param {SubmoduleIgnore} ignore - The ignore rules to follow.
  /// @returns The combination of the `SubmoduleStatus` values.
  ///
  /// @example
  /// ```ts
  /// import { openRepository, submoduleStatusContains, SubmoduleStatus } from 'es-git';
  ///
  /// const repo = await openRepository('...');
  /// const status = repo.submoduleStatus('mysubmodule', 'None');
  ///
  /// console.log(
  ///   submoduleStatusContains(status, SubmoduleStatus.InHead | SubmoduleStatus.InIndex)
  /// ); // true
  /// ```
  pub fn submodule_status(&self, name: String, ignore: SubmoduleIgnore) -> crate::Result<u32> {
    let status = self.inner.submodule_status(&name, ignore.into())?.bits();
    Ok(status)
  }

  #[napi]
  /// Set the ignore rule for the submodule in the configuration
  ///
  /// This does not affect any currently-loaded instances.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submoduleSetIgnore(name: string, ignore: SubmoduleIgnore): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the submodule.
  /// @param {SubmoduleIgnore} ignore - The new value for the ignore rule.
  pub fn submodule_set_ignore(&mut self, name: String, ignore: SubmoduleIgnore) -> crate::Result<()> {
    self.inner.submodule_set_ignore(&name, ignore.into())?;
    Ok(())
  }

  #[napi]
  /// Set the update rule for the submodule in the configuration
  ///
  /// This setting won't affect any existing instances.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submoduleSetUpdate(name: string, update: SubmoduleUpdate): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the submodule.
  /// @param {SubmoduleUpdate} update - The new value to use.
  pub fn submodule_set_update(&mut self, name: String, update: SubmoduleUpdate) -> crate::Result<()> {
    self.inner.submodule_set_update(&name, update.into())?;
    Ok(())
  }

  #[napi]
  /// Set the URL for the submodule in the configuration
  ///
  /// After calling this, you may wish to call `Submodule#sync()` to write
  /// the changes to the checked out submodule repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submoduleSetUrl(name: string, url: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the submodule to configure.
  /// @param {string} url - URL that should be used for the submodule.
  pub fn submodule_set_url(&mut self, name: String, url: String) -> crate::Result<()> {
    self.inner.submodule_set_url(&name, &url)?;
    Ok(())
  }

  #[napi]
  /// Set the branch for the submodule in the configuration
  ///
  /// After calling this, you may wish to call `Submodule#sync()` to write
  /// the changes to the checked out submodule repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   submoduleSetBranch(name: string, branchName: string): void;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the submodule to configure.
  /// @param {string} branchName - Branch that should be used for the submodule
  pub fn submodule_set_branch(&mut self, name: String, branch_name: String) -> crate::Result<()> {
    self.inner.submodule_set_branch(&name, &branch_name)?;
    Ok(())
  }
}
