use crate::util;
use std::path::Path;

use crate::remote::FetchOptions;
use napi::{bindgen_prelude::*, JsString};
use napi_derive::napi;

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

impl From<RepositoryInitOptions> for git2::RepositoryInitOptions {
  fn from(value: RepositoryInitOptions) -> Self {
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

#[napi(string_enum)]
pub enum RepositoryOpenFlags {
  /// Only open the specified path; don't walk upward searching.
  NoSearch,
  /// Search across filesystem boundaries.
  CrossFS,
  /// Force opening as bare repository, and defer loading its config.
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
  #[napi(factory)]
  pub fn init(path: String, options: Option<RepositoryInitOptions>) -> crate::Result<Self> {
    let inner = if let Some(opts) = options {
      git2::Repository::init_opts(path, &opts.into())
    } else {
      git2::Repository::init(path)
    }?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn open(path: String, options: Option<RepositoryOpenOptions>) -> crate::Result<Self> {
    let inner = if let Some(opts) = options {
      let flags = opts.flags.into();
      let ceiling_dirs = opts.ceiling_dirs.unwrap_or_default();
      git2::Repository::open_ext(path, flags, ceiling_dirs)
    } else {
      git2::Repository::open(path)
    }?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn discover(path: String) -> crate::Result<Self> {
    let inner = git2::Repository::discover(path)?;
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn clone(env: Env, url: String, path: String, options: Option<RepositoryCloneOptions>) -> crate::Result<Self> {
    let mut builder = git2::build::RepoBuilder::new();
    let mut recursive = false;
    if let Some(opts) = options {
      if let Some(fetch) = opts.fetch {
        builder.fetch_options(fetch.into_git2_fetch_options(env)?);
      }
      if let Some(true) = opts.recursive {
        recursive = true;
      }
    }
    let this = Self {
      inner: builder.clone(&url, Path::new(&path))?,
    };
    if recursive {
      this.update_submodules()?;
    }
    Ok(this)
  }

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

  #[napi]
  pub fn remote_names(&self) -> crate::Result<Vec<String>> {
    let remotes = self
      .inner
      .remotes()
      .map(|remotes| remotes.into_iter().flatten().map(|x| x.to_owned()).collect::<Vec<_>>())?;
    Ok(remotes)
  }

  fn update_submodules(&self) -> crate::Result<()> {
    fn add_subrepos(repo: &git2::Repository, list: &mut Vec<git2::Repository>) -> crate::Result<()> {
      for mut subm in repo.submodules()? {
        subm.update(true, None)?;
        list.push(subm.open()?);
      }
      Ok(())
    }
    let mut repos = Vec::new();
    add_subrepos(&self.inner, &mut repos)?;
    while let Some(repo) = repos.pop() {
      add_subrepos(&repo, &mut repos)?
    }
    Ok(())
  }
}
