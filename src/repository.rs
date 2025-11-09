use crate::annotated_commit::AnnotatedCommit;
use crate::commit::Commit;
use crate::remote::FetchOptions;
use crate::util;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

#[napi(string_enum)]
/// Available states are `Clean`, `Merge`, `Revert`, `RevertSequence`, `CherryPick`,
/// `CherryPickSequence`, `Bisect`, `Rebase`, `RebaseInteractive`, `RebaseMerge`,
/// `ApplyMailbox`, `ApplyMailboxOrRebase`.
pub enum RepositoryState {
  Clean,
  Merge,
  Revert,
  RevertSequence,
  CherryPick,
  CherryPickSequence,
  Bisect,
  Rebase,
  RebaseInteractive,
  RebaseMerge,
  ApplyMailbox,
  ApplyMailboxOrRebase,
}

impl From<git2::RepositoryState> for RepositoryState {
  fn from(value: git2::RepositoryState) -> Self {
    match value {
      git2::RepositoryState::ApplyMailbox => Self::ApplyMailbox,
      git2::RepositoryState::ApplyMailboxOrRebase => Self::ApplyMailboxOrRebase,
      git2::RepositoryState::Bisect => Self::Bisect,
      git2::RepositoryState::Rebase => Self::Rebase,
      git2::RepositoryState::RebaseInteractive => Self::RebaseInteractive,
      git2::RepositoryState::RebaseMerge => Self::RebaseMerge,
      git2::RepositoryState::CherryPick => Self::CherryPick,
      git2::RepositoryState::CherryPickSequence => Self::CherryPickSequence,
      git2::RepositoryState::Merge => Self::Merge,
      git2::RepositoryState::Revert => Self::Revert,
      git2::RepositoryState::RevertSequence => Self::RevertSequence,
      git2::RepositoryState::Clean => Self::Clean,
    }
  }
}

#[napi]
#[repr(u32)]
/// Mode options for `RepositoryInitOptions`.
pub enum RepositoryInitMode {
  /// Use permissions configured by umask (default)
  SharedUnmask = 0,
  /// Use `--shared=group` behavior, chmod'ing the new repo to be
  /// group writable and "g+sx" for sticky group assignment.
  SharedGroup = 0o002775,
  /// Use `--shared=all` behavior, adding world readability.
  SharedAll = 0o002777,
}

#[napi(object)]
pub struct RepositoryInitOptions {
  /// Create a bare repository with no working directory.
  ///
  /// Defaults to `false`.
  pub bare: Option<bool>,
  /// Return an error if the repository path appears to already be a git
  /// repository.
  ///
  /// Defaults to `false`.
  pub no_reinit: Option<bool>,
  /// Normally a '/.git/' will be appended to the repo path for non-bare repos
  /// (if it is not already there), but passing this flag prevents that
  /// behavior.
  ///
  /// Defaults to `false`.
  pub no_dotgit_dir: Option<bool>,
  /// Make the repo path (and workdir path) as needed. The ".git" directory
  /// will always be created regardless of this flag.
  ///
  /// Defaults to `true`.
  pub mkdir: Option<bool>,
  /// Make the repo path (and workdir path) as needed. The ".git" directory
  /// will always be created regardless of this flag.
  ///
  /// Defaults to `true`.
  pub mkpath: Option<bool>,
  /// Set to one of the `RepositoryInit` constants, or a custom value.
  pub mode: Option<u32>,
  /// Enable or disable using external templates.
  ///
  /// If enabled, then the `template_path` option will be queried first, then
  /// `init.templatedir` from the global config, and finally
  /// `/usr/share/git-core-templates` will be used (if it exists).
  ///
  /// Defaults to `true`.
  pub external_template: Option<bool>,
  /// When the `externalTemplate` option is set, this is the first location
  /// to check for the template directory.
  ///
  /// If this is not configured, then the default locations will be searched
  /// instead.
  pub template_path: Option<String>,
  /// The path to the working directory.
  ///
  /// If this is a relative path it will be evaluated relative to the repo
  /// path. If this is not the "natural" working directory, a .git gitlink
  /// file will be created here linking to the repo path.
  pub workdir_path: Option<String>,
  /// If set, this will be used to initialize the "description" file in the
  /// repository instead of using the template content.
  pub description: Option<String>,
  /// The name of the head to point HEAD at.
  ///
  /// If not configured, this will be taken from your git configuration.
  /// If this begins with `refs/` it will be used verbatim;
  /// otherwise `refs/heads/` will be prefixed.
  pub initial_head: Option<String>,
  /// If set, then after the rest of the repository initialization is
  /// completed an `origin` remote will be added pointing to this URL.
  pub origin_url: Option<String>,
}

impl From<&RepositoryInitOptions> for git2::RepositoryInitOptions {
  fn from(value: &RepositoryInitOptions) -> Self {
    let mut opts = git2::RepositoryInitOptions::new();
    if let Some(bare) = value.bare {
      opts.bare(bare);
    }
    if let Some(no_reinit) = value.no_reinit {
      opts.no_reinit(no_reinit);
    }
    if let Some(no_dotgit_dir) = value.no_dotgit_dir {
      opts.no_dotgit_dir(no_dotgit_dir);
    }
    if let Some(mkdir) = value.mkdir {
      opts.mkdir(mkdir);
    }
    if let Some(mkpath) = value.mkpath {
      opts.mkpath(mkpath);
    }
    if let Some(mode) = value.mode {
      opts.mode(git2::RepositoryInitMode::from_bits_truncate(mode));
    }
    if let Some(external_template) = value.external_template {
      opts.external_template(external_template);
    }
    if let Some(template_path) = &value.template_path {
      opts.template_path(Path::new(template_path));
    }
    if let Some(workdir_path) = &value.workdir_path {
      opts.workdir_path(Path::new(workdir_path));
    }
    if let Some(description) = &value.description {
      opts.description(description);
    }
    if let Some(ref initial_head) = value.initial_head {
      opts.initial_head(initial_head);
    }
    if let Some(ref origin_url) = value.origin_url {
      opts.origin_url(origin_url);
    }
    opts
  }
}

#[napi(object)]
#[derive(Clone, Default)]
pub struct RepositoryOpenOptions {
  /// If this option is `true`, the path must point directly to a repository; otherwise,
  /// this may point to a subdirectory of a repository, and `open` will search up through parent
  /// directories.
  pub no_search: Option<bool>,
  /// If this option is `true`, the search through parent directories will not cross
  /// a filesystem boundary (detected when the stat st_dev field changes).
  pub cross_fs: Option<bool>,
  /// If this option is `true`, force opening the repository as bare event if it isn't, ignoring
  /// any working directory, and defer loading the repository configuration for performance.
  pub bare: Option<bool>,
  /// If this options is `true`, don't try appending `/.git` to `path`.
  pub no_dotgit: Option<bool>,
  /// If this option is `true`, `open` will ignore other options and `ceilingDirs`, and respect
  /// the same environment variables git does.
  /// Note, however, that `path` overrides `$GIT_DIR`.
  pub from_env: Option<bool>,
  /// ceiling_dirs specifies a list of paths that the search through parent
  /// directories will stop before entering.
  pub ceiling_dirs: Option<Vec<String>>,
}

impl RepositoryOpenOptions {
  pub fn flags(&self) -> git2::RepositoryOpenFlags {
    let mut flags = git2::RepositoryOpenFlags::empty();
    if let Some(true) = self.no_search {
      flags.insert(git2::RepositoryOpenFlags::NO_SEARCH);
    }
    if let Some(true) = self.cross_fs {
      flags.insert(git2::RepositoryOpenFlags::CROSS_FS);
    }
    if let Some(true) = self.bare {
      flags.insert(git2::RepositoryOpenFlags::BARE);
    }
    if let Some(true) = self.no_dotgit {
      flags.insert(git2::RepositoryOpenFlags::NO_DOTGIT);
    }
    if let Some(true) = self.from_env {
      flags.insert(git2::RepositoryOpenFlags::FROM_ENV);
    }
    flags
  }
}

#[napi(object)]
pub struct RepositoryCloneOptions {
  /// Indicate whether the repository will be cloned as a bare repository or
  /// not.
  pub bare: Option<bool>,
  /// Specify the name of the branch to check out after the clone.
  ///
  /// If not specified, the remote's default branch will be used.
  pub branch: Option<String>,
  /// Clone a remote repository, initialize and update its submodules
  /// recursively.
  ///
  /// This is similar to `git clone --recursive`.
  pub recursive: Option<bool>,
  /// Options which can be specified to various fetch operations.
  pub fetch: Option<FetchOptions>,
}

#[napi(object)]
pub struct ExtractedSignature {
  /// GPG signature of the commit, or null if the commit is not signed.
  pub signature: String,
  /// Signed data of the commit.
  pub signed_data: String,
}

#[napi]
/// An owned git repository, representing all state associated with the
/// underlying filesystem.
///
/// This class corresponds to a git repository in libgit2.
pub struct Repository {
  pub(crate) inner: git2::Repository,
}

#[napi]
impl Repository {
  #[napi]
  /// Tests whether this repository is a bare repository or not.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   isBare(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if repository is a bare.
  pub fn is_bare(&self) -> bool {
    self.inner.is_bare()
  }

  #[napi]
  /// Tests whether this repository is a shallow clone.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   isShallow(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if repository is a shallow clone.
  pub fn is_shallow(&self) -> bool {
    self.inner.is_shallow()
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
  /// Tests whether this repository is empty.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   isEmpty(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if repository is empty.
  pub fn is_empty(&self) -> crate::Result<bool> {
    Ok(self.inner.is_empty()?)
  }

  #[napi]
  /// Returns the path to the `.git` folder for normal repositories or the
  /// repository itself for bare repositories.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   path(): string;
  /// }
  /// ```
  ///
  /// @returns The path to the `.git` folder for normal repositories or the repository itself
  /// for bare repositories.
  pub fn path(&self) -> String {
    util::path_to_string(self.inner.path())
  }

  #[napi]
  /// Returns the current state of this repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   state(): RepositoryState;
  /// }
  /// ```
  ///
  /// @returns The current state of this repository.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./repo');
  /// console.log(repo.state()); // e.g., 'Clean'
  /// // After a revert/merge/cherry-pick, state can be 'Revert'/'Merge' etc.
  /// // Use repo.cleanupState() to return to 'Clean' when done handling.
  /// ```
  pub fn state(&self) -> RepositoryState {
    self.inner.state().into()
  }

  #[napi]
  /// Get the path of the working directory for this repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   workdir(): string | null;
  /// }
  /// ```
  ///
  /// @returns The path of the working directory for this repository.
  /// If this repository is bare, then `null` is returned.
  /// ```
  pub fn workdir(&self) -> Option<String> {
    self.inner.workdir().map(util::path_to_string)
  }

  #[napi]
  /// Retrieve and resolve the reference pointed at by `HEAD`.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   head(): Reference;
  /// }
  /// ```
  ///
  /// @returns Reference pointed at by `HEAD`.
  pub fn head(&self, this: Reference<Repository>, env: Env) -> crate::Result<crate::reference::Reference> {
    Ok(crate::reference::Reference {
      inner: this.share_with(env, |repo| {
        repo.inner.head().map_err(crate::Error::from).map_err(|e| e.into())
      })?,
    })
  }

  #[napi]
  /// Make the repository `HEAD` point to the specified reference.
  ///
  /// If the provided reference points to a tree or a blob, the `HEAD` is
  /// unaltered and an error is returned.
  ///
  /// If the provided reference points to a branch, the `HEAD` will point to
  /// that branch, staying attached, or become attached if it isn't yet. If
  /// the branch doesn't exist yet, no error will be returned. The `HEAD` will
  /// then be attached to an unborn branch.
  ///
  /// Otherwise, the `HEAD` will be detached and will directly point to the
  /// commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   setHead(refname: string): void;
  /// }
  /// ```
  ///
  /// @param {string} refname - Specified reference to point into `HEAD`.
  pub fn set_head(&self, refname: String) -> crate::Result<()> {
    self.inner.set_head(&refname)?;
    Ok(())
  }

  #[napi]
  /// Determines whether the repository `HEAD` is detached.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   headDetached(): boolean;
  /// }
  /// ```
  ///
  /// @returns Returns `true` if the repository `HEAD` is detached.
  pub fn head_detached(&self) -> crate::Result<bool> {
    Ok(self.inner.head_detached()?)
  }

  #[napi]
  /// Make the repository HEAD directly point to the commit.
  ///
  /// If the provided commitish cannot be found in the repository, the HEAD
  /// is unaltered and an error is returned.
  ///
  /// If the provided commitish cannot be peeled into a commit, the HEAD is
  /// unaltered and an error is returned.
  ///
  /// Otherwise, the HEAD will eventually be detached and will directly point
  /// to the peeled commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   setHeadDetached(commitish: Commit): void;
  /// }
  /// ```
  ///
  /// @param {Commit} commit - A Commit which the HEAD should point to.
  pub fn set_head_detached(&self, commit: &Commit) -> crate::Result<()> {
    let oid = commit.inner.id();
    self.inner.set_head_detached(oid)?;
    Ok(())
  }

  #[napi]
  /// Make the repository HEAD directly point to the commit.
  ///
  /// If the provided commitish cannot be found in the repository, the HEAD
  /// is unaltered and an error is returned.
  /// If the provided commitish cannot be peeled into a commit, the HEAD is
  /// unaltered and an error is returned.
  /// Otherwise, the HEAD will eventually be detached and will directly point
  /// to the peeled commit.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   setHeadDetachedFromAnnotated(commitish: AnnotatedCommit): void;
  /// }
  /// ```
  ///
  /// @param {AnnotatedCommit} commitish - An Annotated Commit which the HEAD should point to.
  pub fn set_head_detached_from_annotated(&self, commitish: &AnnotatedCommit) -> crate::Result<()> {
    let oid = commitish.inner.id();
    let annotated = self.inner.find_annotated_commit(oid)?;
    self.inner.set_head_detached_from_annotated(annotated)?;
    Ok(())
  }

  #[napi]
  /// Extract a signature from an object identified by its ID.
  ///
  /// This method can be used for any object that may be signed, such as commits or tags.
  ///
  /// @category Repository/Methods
  ///
  /// @signature
  /// ```ts
  /// class Repository {
  ///   extractSignature(oid: string): ExtractedSignature | null;
  /// }
  /// ```
  ///
  /// @param {string} oid - Object ID (SHA1) of the signed object to extract the signature from.
  /// @returns An ExtractedSignature object containing the signature and signed data if the object is signed,
  ///          or null if the object is not signed.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('.');
  /// const commit = repo.getCommit('a01e9888e46729ef4aa68953ba19b02a7a64eb82');
  ///
  /// // Extract the signature from a commit
  /// const signatureInfo = repo.extractSignature(commit.id());
  ///
  /// if (signatureInfo) {
  ///   console.log('Object is signed!');
  ///   console.log('Signature:', signatureInfo.signature);
  ///   console.log('Signed data:', signatureInfo.signedData);
  /// } else {
  ///   console.log('Object is not signed');
  /// }
  /// ```
  pub fn extract_signature(&self, oid: String) -> crate::Result<Option<ExtractedSignature>> {
    let oid_obj = git2::Oid::from_str(&oid)?;
    match self.inner.extract_signature(&oid_obj, None) {
      Ok((sig, data)) => {
        let signature = std::str::from_utf8(&sig)?.to_string();
        let signed_data = std::str::from_utf8(&data)?.to_string();
        Ok(Some(ExtractedSignature { signature, signed_data }))
      }
      Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
      Err(e) => Err(crate::Error::from(e)),
    }
  }

  #[napi]
  /// Remove all the metadata associated with an ongoing command like merge,
  /// revert, cherry-pick, etc. For example: `MERGE_HEAD`, `MERGE_MSG`, etc.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   cleanupState(): void;
  /// }
  /// ```
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('./repo');
  /// // After revert or merge operations:
  /// if (repo.state() !== 'Clean') {
  ///   repo.cleanupState();
  /// }
  /// ```
  pub fn cleanup_state(&self) -> crate::Result<()> {
    self.inner.cleanup_state()?;
    Ok(())
  }
}

fn update_submodules(repo: &git2::Repository) -> crate::Result<()> {
  fn add_subrepos(repo: &git2::Repository, list: &mut Vec<git2::Repository>) -> crate::Result<()> {
    for mut subm in repo.submodules()? {
      subm.update(true, None)?;
      list.push(subm.open()?);
    }
    Ok(())
  }
  let mut repos = Vec::new();
  add_subrepos(repo, &mut repos)?;
  while let Some(repo) = repos.pop() {
    add_subrepos(&repo, &mut repos)?;
  }
  Ok(())
}

pub struct InitRepositoryTask {
  path: String,
  options: Option<RepositoryInitOptions>,
}

#[napi]
impl Task for InitRepositoryTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let inner = if let Some(opts) = &self.options {
      git2::Repository::init_opts(&self.path, &opts.into())
    } else {
      git2::Repository::init(&self.path)
    }
    .map_err(crate::Error::from)?;
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
/// Creates a new repository in the specified folder.
///
/// @category Repository
/// @signature
/// ```ts
/// function initRepository(
///   path: string,
///   options?: RepositoryInitOptions | null | undefined,
///   signal?: AbortSignal | null | undefined,
/// ): Promise<Repository>;
/// ```
///
/// @param {string} path - Directory path to create new repository.
/// @param {RepositoryInitOptions} [options] - Options which can be used to configure
/// how a repository is initialized.
/// @param {AbortSignal} [signal] - Abort signal.
///
/// @returns A new repository.
///
/// @example
///
/// Basic example.
///
/// ```ts
/// import { initRepository } from 'es-git';
///
/// const repo = await initRepository('/path/to/repo');
/// ```
///
/// Create bare repository.
///
/// ```ts
/// import { initRepository } from 'es-git';
///
/// const repo = await initRepository('/path/to/repo.git', {
///   bare: true,
/// });
/// ```
pub fn init_repository(
  path: String,
  options: Option<RepositoryInitOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<InitRepositoryTask> {
  AsyncTask::with_optional_signal(InitRepositoryTask { path, options }, signal)
}

pub struct OpenRepositoryTask {
  path: String,
  options: Option<RepositoryOpenOptions>,
}

#[napi]
impl Task for OpenRepositoryTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let options = self.options.to_owned().unwrap_or_default();
    let flags = options.flags();
    let ceiling_dirs = options.ceiling_dirs.to_owned().unwrap_or_default();
    let inner = git2::Repository::open_ext(&self.path, flags, ceiling_dirs).map_err(crate::Error::from)?;
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
/// Attempt to open an already-existing repository at `path`.
///
/// @category Repository
/// @signature
/// ```ts
/// function openRepository(
///   path: string,
///   options?: RepositoryOpenOptions | null | undefined,
///   signal?: AbortSignal | null | undefined,
/// ): Promise<Repository>;
/// ```
///
/// @param {string} path - Directory path to repository already-existing.
/// @param {RepositoryOpenOptions} [options] - Options which can be used to configure
/// how a repository is initialized.
/// @param {AbortSignal} [signal] - Abort signal.
///
/// @returns Opened repository.
///
/// @example
///
/// Basic example.
///
/// ```ts
/// import { openRepository } from 'es-git';
///
/// const repo = await openRepository('/path/to/repo');
/// ```
///
/// Open bare repository.
///
/// ```ts
/// import { openRepository } from 'es-git';
///
/// const repo = await openRepository('/path/to/repo.git', {
///   bare: true,
/// });
/// ```
pub fn open_repository(
  path: String,
  options: Option<RepositoryOpenOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<OpenRepositoryTask> {
  AsyncTask::with_optional_signal(OpenRepositoryTask { path, options }, signal)
}

pub struct DiscoverRepositoryTask {
  path: String,
}

#[napi]
impl Task for DiscoverRepositoryTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let inner = git2::Repository::discover(&self.path).map_err(crate::Error::from)?;
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
/// Attempt to open an already-existing repository at or above `path`.
///
/// This starts at `path` and looks up the filesystem hierarchy
/// until it finds a repository.
///
/// @category Repository
/// @signature
/// ```ts
/// function discoverRepository(path: string, signal?: AbortSignal | null | undefined): Promise<Repository>;
/// ```
///
/// @param {string} path - Directory path to discover repository.
/// @param {AbortSignal} [signal] - Abort signal.
///
/// @returns Git repository.
pub fn discover_repository(path: String, signal: Option<AbortSignal>) -> AsyncTask<DiscoverRepositoryTask> {
  AsyncTask::with_optional_signal(DiscoverRepositoryTask { path }, signal)
}

pub struct CloneRepositoryTask {
  url: String,
  path: String,
  options: Option<RepositoryCloneOptions>,
}

#[napi]
impl Task for CloneRepositoryTask {
  type Output = Repository;
  type JsValue = Repository;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut builder = git2::build::RepoBuilder::new();
    let mut recursive = false;
    if let Some(opts) = &self.options {
      if let Some(bare) = opts.bare {
        builder.bare(bare);
      }
      if let Some(branch) = &opts.branch {
        builder.branch(branch);
      }
      if let Some(fetch) = &opts.fetch {
        let fetch_options = fetch.to_git2_fetch_options();
        builder.fetch_options(fetch_options);
      }
      if let Some(true) = &opts.recursive {
        recursive = true;
      }
    }
    let inner = builder
      .clone(&self.url, Path::new(&self.path))
      .map_err(crate::Error::from)?;
    if recursive {
      update_submodules(&inner)?;
    }
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
/// Clone a remote repository.
///
/// This will use the options configured so far to clone the specified URL
/// into the specified local path.
///
/// @category Repository
///
/// @signature
/// ```ts
/// function cloneRepository(
///   url: string,
///   path: string,
///   options?: RepositoryCloneOptions | null | undefined,
///   signal?: AbortSignal | null | undefined
/// ): Promise<Repository>;
/// ```
///
/// @param {string} url - Remote URL for repository.
/// @param {string} path - Local path to clone repository.
/// @param {RepositoryCloneOptions|undefined|null} [options] - Clone options for repository.
/// @param {AbortSignal|undefined|null} [signal] - Abort signal.
/// @returns Repository instance
///
/// @example
///
/// Clone repository using `https://` protocol.
///
/// ```ts
/// import { cloneRepository } from 'es-git';
///
/// const repo = await cloneRepository(
///   'https://github.com/toss/es-git',
///   '/path/to/clone',
/// );
/// ```
///
/// Clone repository using `git://` protocol.
///
/// ```ts
/// import { cloneRepository } from 'es-git';
///
/// const repo = await cloneRepository(
///   'git@github.com:toss/es-git',
///   '/path/to/clone',
/// );
/// ```
///
/// Clone repository with authentication.
///
/// ```ts
/// import { cloneRepository } from 'es-git';
///
/// // Authenticate using ssh-agent
/// const repo = await cloneRepository('git@github.com:toss/es-git', '.', {
///   fetch: {
///     credential: {
///       type: 'SSHKeyFromAgent',
///     },
///   },
/// });
/// ```
pub fn clone_repository(
  url: String,
  path: String,
  options: Option<RepositoryCloneOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<CloneRepositoryTask> {
  AsyncTask::with_optional_signal(CloneRepositoryTask { url, path, options }, signal)
}
