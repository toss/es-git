use napi::bindgen_prelude::*;
use napi::JsUndefined;
use napi_derive::napi;
use std::path::Path;

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
  pub(crate) fn into_git2_cred(self) -> std::result::Result<git2::Cred, git2::Error> {
    let fallback = "git";
    match self.r#type {
      CredentialType::Default => git2::Cred::default(),
      CredentialType::SSHKeyFromAgent => {
        git2::Cred::ssh_key_from_agent(self.username.unwrap_or(fallback.to_string()).as_ref())
      }
      CredentialType::SSHKeyFromPath => git2::Cred::ssh_key(
        self.username.unwrap_or(fallback.to_string()).as_ref(),
        self.public_key_path.as_ref().map(Path::new),
        Path::new(&self.private_key_path.unwrap()),
        self.passphrase.as_ref().map(String::as_ref),
      ),
      CredentialType::SSHKey => git2::Cred::ssh_key_from_memory(
        self.username.unwrap_or(fallback.to_string()).as_ref(),
        self.public_key.as_ref().map(String::as_ref),
        &self.private_key.unwrap(),
        self.passphrase.as_ref().map(String::as_ref),
      ),
      CredentialType::Plain => git2::Cred::userpass_plaintext(
        self.username.unwrap_or(fallback.to_string()).as_ref(),
        &self.password.unwrap(),
      ),
    }
  }
}

#[napi(object)]
pub struct ProxyOptions {
  pub auto: Option<bool>,
  pub url: Option<String>,
}

impl ProxyOptions {
  pub(crate) fn into_git2_proxy_options(self) -> git2::ProxyOptions<'static> {
    let mut proxy_options = git2::ProxyOptions::new();
    if let Some(true) = self.auto {
      proxy_options.auto();
    }
    if let Some(url) = self.url {
      proxy_options.url(&url);
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
pub struct Progress {
  pub total_objects: u32,
  pub indexed_objects: u32,
  pub received_objects: u32,
  pub local_objects: u32,
  pub total_deltas: u32,
  pub indexed_deltas: u32,
  pub received_bytes: u32,
}

impl<'a> From<git2::Progress<'a>> for Progress {
  fn from(progress: git2::Progress) -> Self {
    Progress {
      total_objects: progress.total_objects() as u32,
      indexed_objects: progress.indexed_objects() as u32,
      received_objects: progress.received_objects() as u32,
      local_objects: progress.local_objects() as u32,
      total_deltas: progress.total_deltas() as u32,
      indexed_deltas: progress.indexed_deltas() as u32,
      received_bytes: progress.received_bytes() as u32,
    }
  }
}

#[napi(object)]
pub struct PushProgress {
  pub current: u32,
  pub total: u32,
  pub bytes: u32,
}

#[napi(object)]
pub struct FetchOptions {
  pub credential: Option<Credential>,
  pub proxy: Option<ProxyOptions>,
  pub prune: Option<FetchPrune>,
  pub depth: Option<i32>,
  pub download_tags: Option<AutotagOption>,
  pub follow_redirects: Option<RemoteRedirect>,
  pub custom_headers: Option<Vec<String>>,
  #[napi(ts_type = "(progress: Progress) => void")]
  pub on_transfer_progress: Option<Function<'static, Progress, JsUndefined>>,
  #[napi(ts_type = "(progress: PushProgress) => void")]
  pub on_push_transfer_progress: Option<Function<'static, PushProgress, JsUndefined>>,
}

#[napi]
impl FetchOptions {
  pub(crate) fn into_git2_fetch_options(self, env: Env) -> crate::Result<git2::FetchOptions<'static>> {
    let mut fetch = git2::FetchOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    if let Some(credential) = self.credential {
      callbacks.credentials(move |_url, _username, _cred| credential.clone().into_git2_cred());
    }
    if let Some(on_transfer_progress) = self.on_transfer_progress {
      let func_ref = on_transfer_progress.create_ref()?;
      callbacks.transfer_progress(move |p| func_ref.borrow_back(&env).and_then(|cb| cb.call(p.into())).is_ok());
    }
    if let Some(on_push_transfer_progress) = self.on_push_transfer_progress {
      let func_ref = on_push_transfer_progress.create_ref()?;
      callbacks.push_transfer_progress(move |current, total, bytes| {
        let _ = func_ref.borrow_back(&env).and_then(|cb| {
          cb.call(PushProgress {
            current: current as u32,
            total: total as u32,
            bytes: bytes as u32,
          })
        });
      });
    }
    fetch.remote_callbacks(callbacks);
    if let Some(proxy) = self.proxy {
      fetch.proxy_options(proxy.into_git2_proxy_options());
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
    if let Some(custom_headers) = self.custom_headers {
      fetch.custom_headers(&custom_headers.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    }
    Ok(fetch)
  }
}
