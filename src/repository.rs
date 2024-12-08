use crate::remote::FetchOptions;
use crate::util;
use napi::{bindgen_prelude::*, JsString};
use napi_derive::napi;
use std::path::Path;

#[napi(string_enum)]
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

#[napi(object)]
pub struct RepositoryInitOptions {
  pub bare: Option<bool>,
  pub initial_head: Option<String>,
  pub origin_url: Option<String>,
}

impl From<&RepositoryInitOptions> for git2::RepositoryInitOptions {
  fn from(value: &RepositoryInitOptions) -> Self {
    let mut opts = git2::RepositoryInitOptions::new();
    if let Some(bare) = value.bare {
      opts.bare(bare);
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
pub struct RepositoryOpenOptions {
  pub flags: RepositoryOpenFlags,
  pub ceiling_dirs: Option<Vec<String>>,
}

#[napi]
pub enum RepositoryOpenFlags {
  /// Only open the specified path; don't walk upward searching.
  NoSearch,
  /// Search across filesystem boundaries.
  CrossFS,
  /// Force opening as a bare repository, and defer loading its config.
  Bare,
  /// Don't try appending `/.git` to the specified repository path.
  NoDotGit,
  /// Respect environment variables like `$GIT_DIR`.
  FromEnv,
}

impl From<RepositoryOpenFlags> for git2::RepositoryOpenFlags {
  fn from(val: RepositoryOpenFlags) -> Self {
    match val {
      RepositoryOpenFlags::NoSearch => git2::RepositoryOpenFlags::NO_SEARCH,
      RepositoryOpenFlags::CrossFS => git2::RepositoryOpenFlags::CROSS_FS,
      RepositoryOpenFlags::Bare => git2::RepositoryOpenFlags::BARE,
      RepositoryOpenFlags::NoDotGit => git2::RepositoryOpenFlags::NO_DOTGIT,
      RepositoryOpenFlags::FromEnv => git2::RepositoryOpenFlags::FROM_ENV,
    }
  }
}

#[napi(object)]
pub struct RepositoryCloneOptions {
  pub recursive: Option<bool>,
  pub fetch: Option<FetchOptions>,
}

#[napi]
pub struct Repository {
  pub(crate) inner: git2::Repository,
}

#[napi]
impl Repository {
  #[napi]
  pub fn is_bare(&self) -> bool {
    self.inner.is_bare()
  }

  #[napi]
  pub fn is_shallow(&self) -> bool {
    self.inner.is_shallow()
  }

  #[napi]
  pub fn is_worktree(&self) -> bool {
    self.inner.is_worktree()
  }

  #[napi]
  pub fn is_empty(&self) -> crate::Result<bool> {
    Ok(self.inner.is_empty()?)
  }

  #[napi]
  pub fn path(&self, env: Env) -> crate::Result<JsString> {
    let path = util::path_to_js_string(&env, self.inner.path())?;
    Ok(path)
  }

  #[napi]
  pub fn state(&self) -> RepositoryState {
    self.inner.state().into()
  }

  #[napi]
  pub fn workdir(&self, env: Env) -> Option<JsString> {
    self
      .inner
      .workdir()
      .and_then(|path| util::path_to_js_string(&env, path).ok())
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
    add_subrepos(&repo, &mut repos)?
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
    let inner = if let Some(opts) = &self.options {
      let flags = opts.flags.to_owned().into();
      let ceiling_dirs = opts.ceiling_dirs.to_owned().unwrap_or_default();
      git2::Repository::open_ext(&self.path, flags, ceiling_dirs)
    } else {
      git2::Repository::open(&self.path)
    }
    .map_err(crate::Error::from)?;
    Ok(Repository { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
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
pub fn clone_repository(
  url: String,
  path: String,
  options: Option<RepositoryCloneOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<CloneRepositoryTask> {
  AsyncTask::with_optional_signal(CloneRepositoryTask { url, path, options }, signal)
}
