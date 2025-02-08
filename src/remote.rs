use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi::JsString;
use napi_derive::napi;
use std::path::Path;
use std::sync::RwLock;

#[napi(string_enum)]
pub enum Direction {
  Fetch,
  Push,
}

impl From<git2::Direction> for Direction {
  fn from(value: git2::Direction) -> Self {
    match value {
      git2::Direction::Fetch => Direction::Fetch,
      git2::Direction::Push => Direction::Push,
    }
  }
}

#[napi(object)]
pub struct RefspecObject {
  pub direction: Direction,
  pub src: String,
  pub dst: String,
  pub force: bool,
}

impl<'a> TryFrom<git2::Refspec<'a>> for RefspecObject {
  type Error = crate::Error;

  fn try_from(value: git2::Refspec<'a>) -> std::result::Result<Self, Self::Error> {
    let src = std::str::from_utf8(value.src_bytes())?.to_string();
    let dst = std::str::from_utf8(value.dst_bytes())?.to_string();
    Ok(Self {
      direction: value.direction().into(),
      src,
      dst,
      force: value.is_force(),
    })
  }
}

#[napi(string_enum)]
pub enum CredentialType {
  Default,
  SSHKeyFromAgent,
  SSHKeyFromPath,
  SSHKey,
  Plain,
}

#[napi(object)]
#[derive(Clone)]
pub struct Credential {
  pub r#type: CredentialType,
  pub username: Option<String>,
  pub public_key_path: Option<String>,
  pub public_key: Option<String>,
  pub private_key_path: Option<String>,
  pub private_key: Option<String>,
  pub passphrase: Option<String>,
  pub password: Option<String>,
}

impl Credential {
  pub(crate) fn to_git2_cred(&self) -> std::result::Result<git2::Cred, git2::Error> {
    let fallback = "git".to_string();
    let cred = match self.r#type {
      CredentialType::Default => git2::Cred::default(),
      CredentialType::SSHKeyFromAgent => {
        git2::Cred::ssh_key_from_agent(self.username.to_owned().unwrap_or(fallback).as_ref())
      }
      CredentialType::SSHKeyFromPath => git2::Cred::ssh_key(
        self.username.to_owned().unwrap_or(fallback).as_ref(),
        self.public_key_path.as_ref().map(Path::new),
        Path::new(&self.private_key_path.to_owned().unwrap()),
        self.passphrase.as_ref().map(String::as_ref),
      ),
      CredentialType::SSHKey => git2::Cred::ssh_key_from_memory(
        self.username.to_owned().unwrap_or(fallback).as_ref(),
        self.public_key.as_ref().map(String::as_ref),
        &self.private_key.to_owned().unwrap(),
        self.passphrase.as_ref().map(String::as_ref),
      ),
      CredentialType::Plain => git2::Cred::userpass_plaintext(
        self.username.to_owned().unwrap_or(fallback).as_ref(),
        &self.password.to_owned().unwrap(),
      ),
    }?;
    Ok(cred)
  }
}

#[napi(object)]
/// Options which can be specified to various fetch operations.
pub struct ProxyOptions {
  /// Try to auto-detect the proxy from the git configuration.
  ///
  /// Note that this will override `url` specified before.
  pub auto: Option<bool>,
  /// Specify the exact URL of the proxy to use.
  ///
  /// Note that this will override `auto` specified before.
  pub url: Option<String>,
}

impl ProxyOptions {
  pub(crate) fn to_git2_proxy_options(&self) -> git2::ProxyOptions<'static> {
    let mut proxy_options = git2::ProxyOptions::new();
    if let Some(true) = self.auto {
      proxy_options.auto();
    }
    if let Some(url) = &self.url {
      proxy_options.url(url);
    }
    proxy_options
  }
}

#[napi(string_enum)]
pub enum FetchPrune {
  /// Use the setting from the configuration
  Unspecified,
  /// Force pruning on
  On,
  /// Force pruning off
  Off,
}

impl From<FetchPrune> for git2::FetchPrune {
  fn from(value: FetchPrune) -> Self {
    match value {
      FetchPrune::Unspecified => git2::FetchPrune::Unspecified,
      FetchPrune::On => git2::FetchPrune::On,
      FetchPrune::Off => git2::FetchPrune::Off,
    }
  }
}

#[napi(string_enum)]
/// Automatic tag following options.
pub enum AutotagOption {
  /// Use the setting from the remote's configuration
  Unspecified,
  /// Ask the server for tags pointing to objects we're already downloading
  Auto,
  /// Don't ask for any tags beyond the refspecs
  None,
  /// Ask for all the tags
  All,
}

impl From<AutotagOption> for git2::AutotagOption {
  fn from(value: AutotagOption) -> Self {
    match value {
      AutotagOption::Unspecified => git2::AutotagOption::Unspecified,
      AutotagOption::Auto => git2::AutotagOption::Auto,
      AutotagOption::None => git2::AutotagOption::None,
      AutotagOption::All => git2::AutotagOption::All,
    }
  }
}

#[napi(string_enum)]
/// Remote redirection settings; whether redirects to another host are
/// permitted.
///
/// By default, git will follow a redirect on the initial request
/// (`/info/refs`), but not subsequent requests.
pub enum RemoteRedirect {
  /// Do not follow any off-site redirects at any stage of the fetch or push.
  None,
  /// Allow off-site redirects only upon the initial request. This is the
  /// default.
  Initial,
  /// Allow redirects at any stage in the fetch or push.
  All,
}

impl From<RemoteRedirect> for git2::RemoteRedirect {
  fn from(value: RemoteRedirect) -> Self {
    match value {
      RemoteRedirect::None => git2::RemoteRedirect::None,
      RemoteRedirect::Initial => git2::RemoteRedirect::Initial,
      RemoteRedirect::All => git2::RemoteRedirect::All,
    }
  }
}

#[napi(object)]
/// Options which can be specified to various fetch operations.
pub struct FetchOptions {
  pub credential: Option<Credential>,
  /// Set the proxy options to use for the fetch operation.
  pub proxy: Option<ProxyOptions>,
  /// Set whether to perform a prune after the fetch.
  pub prune: Option<FetchPrune>,
  /// Set fetch depth, a value less or equal to 0 is interpreted as pull
  /// everything (effectively the same as not declaring a limit depth).
  pub depth: Option<i32>,
  /// Set how to behave regarding tags on the remote, such as auto-downloading
  /// tags for objects we're downloading or downloading all of them.
  ///
  /// The default is to auto-follow tags.
  pub download_tags: Option<AutotagOption>,
  /// Set remote redirection settings; whether redirects to another host are
  /// permitted.
  ///
  /// By default, git will follow a redirect on the initial request
  /// (`/info/refs`), but not subsequent requests.
  pub follow_redirects: Option<RemoteRedirect>,
  /// Set extra headers for this fetch operation.
  pub custom_headers: Option<Vec<String>>,
}

impl<'a> FetchOptions {
  pub(crate) fn to_git2_fetch_options(&'a self) -> git2::FetchOptions<'a> {
    let mut fetch = git2::FetchOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    if let Some(cred) = &self.credential {
      callbacks.credentials(move |_url, _username, _cred| cred.to_git2_cred());
    }
    fetch.remote_callbacks(callbacks);
    if let Some(proxy) = &self.proxy {
      fetch.proxy_options(proxy.to_git2_proxy_options());
    }
    if let Some(prune) = self.prune {
      fetch.prune(prune.into());
    }
    if let Some(depth) = self.depth {
      fetch.depth(depth);
    }
    if let Some(download_tags) = self.download_tags {
      fetch.download_tags(download_tags.into());
    }
    if let Some(follow_redirects) = self.follow_redirects {
      fetch.follow_redirects(follow_redirects.into());
    }
    if let Some(custom_headers) = &self.custom_headers {
      fetch.custom_headers(&custom_headers.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    }
    fetch
  }
}

#[napi(object)]
/// Options to control the behavior of a git push.
pub struct PushOptions {
  pub credential: Option<Credential>,
  /// Set the proxy options to use for the push operation.
  pub proxy: Option<ProxyOptions>,
  /// If the transport being used to push to the remote requires the creation
  /// of a pack file, this controls the number of worker threads used by the
  /// packbuilder when creating that pack file to be sent to the remote.
  ///
  /// If set to 0, the packbuilder will auto-detect the number of threads to
  /// create, and the default value is 1.
  pub pb_parallelism: Option<u32>,
  /// Set remote redirection settings; whether redirects to another host are
  /// permitted.
  ///
  /// By default, git will follow a redirect on the initial request
  /// (`/info/refs`), but not subsequent requests.
  pub follow_redirects: Option<RemoteRedirect>,
  /// Set extra headers for this push operation.
  pub custom_headers: Option<Vec<String>>,
  /// Set "push options" to deliver to the remote.
  pub remote_options: Option<Vec<String>>,
}

impl<'a> PushOptions {
  pub(crate) fn to_git2_push_options(&'a self) -> git2::PushOptions<'a> {
    let mut push = git2::PushOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    if let Some(cred) = &self.credential {
      callbacks.credentials(move |_url, _username, _cred| cred.to_git2_cred());
    }
    push.remote_callbacks(callbacks);
    if let Some(proxy) = &self.proxy {
      push.proxy_options(proxy.to_git2_proxy_options());
    }
    if let Some(pb_parallelism) = self.pb_parallelism {
      push.packbuilder_parallelism(pb_parallelism);
    }
    if let Some(follow_redirects) = self.follow_redirects {
      push.follow_redirects(follow_redirects.into());
    }
    if let Some(custom_headers) = &self.custom_headers {
      push.custom_headers(&custom_headers.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    }
    if let Some(remote_options) = &self.remote_options {
      push.remote_push_options(&remote_options.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    }
    push
  }
}

#[napi(object)]
pub struct CreateRemoteOptions {
  pub fetch_refspec: Option<String>,
}

#[napi(object)]
pub struct FetchRemoteOptions {
  pub fetch: Option<FetchOptions>,
  pub reflog_msg: Option<String>,
}

#[napi(object)]
pub struct PruneOptions {
  pub credential: Option<Credential>,
}

pub struct FetchRemoteTask {
  remote: RwLock<Reference<Remote>>,
  refspecs: Vec<String>,
  options: Option<FetchRemoteOptions>,
}

unsafe impl Send for FetchRemoteTask {}

#[napi]
impl Task for FetchRemoteTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let mut remote = self
      .remote
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut fetch_options = match &self.options {
      Some(FetchRemoteOptions { fetch: Some(fetch), .. }) => Some(fetch.to_git2_fetch_options()),
      _ => None,
    };
    let reflog_msg = match &self.options {
      Some(FetchRemoteOptions {
        reflog_msg: Some(reflog_msg),
        ..
      }) => Some(reflog_msg.as_str()),
      _ => None,
    };
    remote
      .inner
      .fetch(&self.refspecs, fetch_options.as_mut(), reflog_msg)
      .map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct PushRemoteTask {
  remote: RwLock<Reference<Remote>>,
  refspecs: Vec<String>,
  options: Option<PushOptions>,
}

unsafe impl Send for PushRemoteTask {}

#[napi]
impl Task for PushRemoteTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let mut remote = self
      .remote
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut push_options = self.options.as_ref().map(|x| x.to_git2_push_options());
    remote
      .inner
      .push(&self.refspecs, push_options.as_mut())
      .map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct PruneRemoteTask {
  remote: RwLock<Reference<Remote>>,
  options: Option<PruneOptions>,
}

unsafe impl Send for PruneRemoteTask {}

#[napi]
impl Task for PruneRemoteTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let mut remote = self
      .remote
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let callbacks = match &self.options {
      Some(PruneOptions {
        credential: Some(cred), ..
      }) => {
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |_url, _username, _cred| cred.to_git2_cred());
        Some(callbacks)
      }
      _ => None,
    };
    remote.inner.prune(callbacks).map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct GetRemoteDefaultBranchTask {
  remote: RwLock<Reference<Remote>>,
}

unsafe impl Send for GetRemoteDefaultBranchTask {}

#[napi]
impl Task for GetRemoteDefaultBranchTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut remote = self
      .remote
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    remote
      .inner
      .connect(git2::Direction::Fetch)
      .map_err(crate::Error::from)?;
    let buf = remote.inner.default_branch().map_err(crate::Error::from)?;
    let branch = std::str::from_utf8(&buf).map_err(crate::Error::from)?.to_string();
    remote.inner.disconnect().map_err(crate::Error::from)?;
    Ok(branch)
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(&output)
  }
}

#[napi]
/// A structure representing a [remote][1] of a git repository.
/// @hideconstructor
///
/// [1]: http://git-scm.com/book/en/Git-Basics-Working-with-Remotes
pub struct Remote {
  pub(crate) inner: SharedReference<Repository, git2::Remote<'static>>,
}

#[napi]
impl Remote {
  #[napi]
  /// Get the remote's name.
  ///
  /// Returns `null` if this remote has not yet been named, and
  /// Throws error if the URL is not valid utf-8
  pub fn name(&self) -> crate::Result<Option<String>> {
    let name = match self.inner.name_bytes() {
      Some(bytes) => Some(std::str::from_utf8(bytes)?.to_string()),
      None => None,
    };
    Ok(name)
  }

  #[napi]
  /// Get the remote's URL.
  ///
  /// Throws error if the URL is not valid utf-8
  pub fn url(&self) -> crate::Result<String> {
    let url = std::str::from_utf8(self.inner.url_bytes())?.to_string();
    Ok(url)
  }

  #[napi]
  /// Get the remote's URL.
  ///
  /// Returns `null` if push url not exists, and
  /// Throws error if the URL is not valid utf-8
  pub fn pushurl(&self) -> crate::Result<Option<String>> {
    let pushurl = match self.inner.pushurl_bytes() {
      Some(bytes) => Some(std::str::from_utf8(bytes)?.to_string()),
      None => None,
    };
    Ok(pushurl)
  }

  #[napi]
  /// List all refspecs.
  ///
  /// Filter refspec if has not valid src or dst with utf-8
  pub fn refspecs(&self) -> Vec<RefspecObject> {
    self
      .inner
      .refspecs()
      .filter_map(|x| RefspecObject::try_from(x).ok())
      .collect::<Vec<_>>()
  }

  #[napi]
  /// Download new data and update tips
  ///
  /// Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.
  pub fn fetch(
    &self,
    self_ref: Reference<Remote>,
    refspecs: Vec<String>,
    options: Option<FetchRemoteOptions>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<FetchRemoteTask> {
    AsyncTask::with_optional_signal(
      FetchRemoteTask {
        remote: RwLock::new(self_ref),
        refspecs,
        options,
      },
      signal,
    )
  }

  #[napi]
  /// Perform a push
  ///
  /// Perform all the steps for a push.
  /// If no refspecs are passed, then the configured refspecs will be used.
  pub fn push(
    &self,
    self_ref: Reference<Remote>,
    refspecs: Vec<String>,
    options: Option<PushOptions>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<PushRemoteTask> {
    AsyncTask::with_optional_signal(
      PushRemoteTask {
        remote: RwLock::new(self_ref),
        refspecs,
        options,
      },
      signal,
    )
  }

  #[napi]
  /// Prune tracking refs that are no longer present on remote
  pub fn prune(
    &self,
    self_ref: Reference<Remote>,
    options: Option<PruneOptions>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<PruneRemoteTask> {
    AsyncTask::with_optional_signal(
      PruneRemoteTask {
        remote: RwLock::new(self_ref),
        options,
      },
      signal,
    )
  }

  #[napi]
  /// Get the remote’s default branch.
  pub fn default_branch(
    &self,
    self_ref: Reference<Remote>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<GetRemoteDefaultBranchTask> {
    AsyncTask::with_optional_signal(
      GetRemoteDefaultBranchTask {
        remote: RwLock::new(self_ref),
      },
      signal,
    )
  }
}

#[napi]
impl Repository {
  #[napi]
  /// List all remotes for a given repository
  pub fn remote_names(&self) -> crate::Result<Vec<String>> {
    let remotes = self
      .inner
      .remotes()
      .map(|names| names.into_iter().flatten().map(|x| x.to_owned()).collect::<Vec<_>>())?;
    Ok(remotes)
  }

  #[napi]
  /// Get remote from repository
  ///
  /// Throws error if not exists
  pub fn get_remote(&self, this: Reference<Repository>, env: Env, name: String) -> crate::Result<Remote> {
    let remote = Remote {
      inner: this.share_with(env, move |repo| {
        repo
          .inner
          .find_remote(&name)
          .map_err(crate::Error::from)
          .map_err(|e| e.into())
      })?,
    };
    Ok(remote)
  }

  #[napi]
  /// Find remote from repository
  pub fn find_remote(&self, this: Reference<Repository>, env: Env, name: String) -> Option<Remote> {
    self.get_remote(this, env, name).ok()
  }

  #[napi]
  /// Add a remote with the default fetch refspec to the repository’s configuration.
  pub fn create_remote(
    &self,
    this: Reference<Repository>,
    env: Env,
    name: String,
    url: String,
    options: Option<CreateRemoteOptions>,
  ) -> crate::Result<Remote> {
    let remote = Remote {
      inner: this.share_with(env, move |repo| {
        if let Some(CreateRemoteOptions {
          fetch_refspec: Some(refspec),
        }) = options
        {
          repo.inner.remote_with_fetch(&name, &url, &refspec)
        } else {
          repo.inner.remote(&name, &url)
        }
        .map_err(crate::Error::from)
        .map_err(|e| e.into())
      })?,
    };
    Ok(remote)
  }
}
