use crate::js::{JsCallback, JsCallbackExt};
use crate::repository::Repository;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{JsValue, ValueType};
use napi_derive::napi;
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;

#[napi(string_enum)]
/// - `Fetch` : Fetch direction.
/// - `Push` : Push direction.
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
/// A data object to represent a git [refspec][1].
///
/// Refspecs are currently mainly accessed/created through a `Remote`.
///
/// [1]: https://git-scm.com/book/en/Git-Internals-The-Refspec
pub struct Refspec {
  pub direction: Direction,
  /// The source specifier.
  pub src: String,
  /// The destination specifier.
  pub dst: String,
  /// Force update setting.
  pub force: bool,
}

impl<'a> TryFrom<git2::Refspec<'a>> for Refspec {
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
#[derive(Copy, Clone, Debug)]
pub enum CredentialType {
  Default,
  SSHKeyFromAgent,
  SSHKeyFromPath,
  SSHKey,
  Plain,
}

impl CredentialType {
  const ALL: [CredentialType; 5] = [
    CredentialType::Default,
    CredentialType::SSHKeyFromAgent,
    CredentialType::SSHKeyFromPath,
    CredentialType::SSHKey,
    CredentialType::Plain,
  ];

  fn to_git2(self) -> git2::CredentialType {
    match self {
      CredentialType::Default => git2::CredentialType::DEFAULT,
      CredentialType::SSHKeyFromAgent | CredentialType::SSHKeyFromPath => git2::CredentialType::SSH_KEY,
      CredentialType::SSHKey => git2::CredentialType::SSH_MEMORY,
      CredentialType::Plain => git2::CredentialType::USER_PASS_PLAINTEXT,
    }
  }

  fn allowed_by(allowed: git2::CredentialType) -> Vec<CredentialType> {
    Self::ALL
      .into_iter()
      .filter(|x| allowed.intersects(x.to_git2()))
      .collect()
  }
}

#[napi(object)]
#[derive(Clone)]
/// A interface to represent git credentials in libgit2.
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

/// How many times a credential callback may be called for a single operation.
///
/// libgit2 asks again whenever the remote rejects a credential, and its SSH transport never gives up
/// on its own, so a callback that keeps returning the same rejected credential would loop forever.
const MAX_CREDENTIAL_ATTEMPTS: usize = 10;

/// How often a worker thread waiting for a credential checks whether the callback was released.
const CREDENTIAL_POLL_INTERVAL: Duration = Duration::from_millis(100);

#[napi(object, object_from_js = false, use_nullable = true)]
/// Arguments passed to a credential callback.
pub struct CredentialCallbackArgs {
  /// URL of the remote that asks for authentication.
  pub url: String,
  /// Username embedded in the URL, such as `git` in `ssh://git@github.com/toss/es-git`.
  /// `null` if the URL has none.
  pub username_from_url: Option<String>,
  /// Credential types the remote accepts. The returned credential must be one of these.
  ///
  /// SSH remotes without a username in the URL first ask for the username alone; this list is
  /// empty then, and only the `username` of the returned credential is used.
  #[napi(ts_type = "CredentialType[]")]
  pub allowed_types: Vec<CredentialType>,
}

type CredentialAnswer = std::result::Result<Option<Credential>, String>;

/// A request for a credential, sent from a libgit2 worker thread to the JavaScript thread.
pub struct CredentialRequest {
  args: CredentialCallbackArgs,
  sender: mpsc::Sender<CredentialAnswer>,
}

/// Calls the user's credential function on the JavaScript thread.
///
/// The threadsafe function wraps a no-op; the user's function is called by our own callback instead,
/// so that whatever it throws (including non-`Error` values) is caught here and turned into an answer.
pub type CredentialCallback = Arc<ThreadsafeFunction<CredentialRequest, (), (), Status, false, true>>;

/// Either a static `Credential` or a function returning one, accepted by `credential` options.
pub enum CredentialOption {
  Static(Credential),
  Callback(CredentialCallback),
}

impl TypeName for CredentialOption {
  fn type_name() -> &'static str {
    "Credential | CredentialCallback"
  }

  fn value_type() -> ValueType {
    ValueType::Unknown
  }
}

impl FromNapiValue for CredentialOption {
  unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> Result<Self> {
    let value = unsafe { Unknown::from_napi_value(env, napi_val) }?;
    if value.get_type()? == ValueType::Function {
      let user_fn = unsafe { FunctionRef::<CredentialCallbackArgs, Unknown>::from_napi_value(env, napi_val) }?;
      let env = Env::from_raw(env);
      let noop = env.create_function_from_closure::<(), (), _>("credential", |_| Ok(()))?;
      let callback = noop
        .build_threadsafe_function::<CredentialRequest>()
        .callee_handled::<false>()
        .weak::<true>()
        .build_callback(move |ctx| {
          call_credential_fn(&ctx.env, &user_fn, ctx.value);
          Ok(())
        })?;
      return Ok(Self::Callback(Arc::new(callback)));
    }
    let credential = unsafe { Credential::from_napi_value(env, napi_val) }?;
    Ok(Self::Static(credential))
  }
}

impl CredentialOption {
  pub(crate) fn install<'a>(&'a self, callbacks: &mut git2::RemoteCallbacks<'a>) {
    match self {
      Self::Static(cred) => {
        callbacks.credentials(move |_url, _username, _cred| cred.to_git2_cred());
      }
      Self::Callback(callback) => {
        let mut attempts = 0;
        callbacks.credentials(move |url, username_from_url, allowed| {
          attempts += 1;
          if attempts > MAX_CREDENTIAL_ATTEMPTS {
            return Err(credential_error(format!(
              "credential callback was called {MAX_CREDENTIAL_ATTEMPTS} times for {url} without a successful authentication; giving up"
            )));
          }
          request_credential(callback, url, username_from_url, allowed)
        });
      }
    }
  }
}

/// Ask the JavaScript callback for a credential and block until it answers.
///
/// This runs on the libuv worker thread executing the git operation, never on the JavaScript thread,
/// so blocking here leaves the JavaScript thread free to run the callback and settle its promise.
fn request_credential(
  callback: &CredentialCallback,
  url: &str,
  username_from_url: Option<&str>,
  allowed: git2::CredentialType,
) -> std::result::Result<git2::Cred, git2::Error> {
  let args = CredentialCallbackArgs {
    url: url.to_string(),
    username_from_url: username_from_url.map(str::to_string),
    allowed_types: CredentialType::allowed_by(allowed),
  };
  let (sender, receiver) = mpsc::channel::<CredentialAnswer>();
  let status = callback.call(
    CredentialRequest { args, sender },
    ThreadsafeFunctionCallMode::NonBlocking,
  );
  if status != Status::Ok {
    return Err(credential_error(format!(
      "failed to call the credential callback: {status}"
    )));
  }
  let answer = loop {
    match receiver.recv_timeout(CREDENTIAL_POLL_INTERVAL) {
      Ok(answer) => break answer,
      Err(mpsc::RecvTimeoutError::Timeout) if callback.aborted() => {
        return Err(credential_error("credential callback was released before it answered"));
      }
      Err(mpsc::RecvTimeoutError::Timeout) => continue,
      Err(mpsc::RecvTimeoutError::Disconnected) => {
        return Err(credential_error("credential callback did not answer"));
      }
    }
  };
  let cred = answer
    .map_err(credential_error)?
    .ok_or_else(|| credential_error(format!("credential callback returned no credential for {url}")))?;
  if !allowed.intersects(cred.r#type.to_git2()) {
    if allowed.contains(git2::CredentialType::USERNAME) {
      return git2::Cred::username(cred.username.as_deref().unwrap_or("git"));
    }
    let accepted = CredentialType::allowed_by(allowed)
      .iter()
      .map(|x| format!("{x:?}"))
      .collect::<Vec<_>>();
    let accepted = match accepted.is_empty() {
      true => "no supported credential type".to_string(),
      false => format!("only {}", accepted.join(", ")),
    };
    return Err(credential_error(format!(
      "credential callback returned a {:?} credential, but {url} accepts {accepted}",
      cred.r#type
    )));
  }
  if let Some(field) = missing_required_field(&cred) {
    return Err(credential_error(format!(
      "credential callback returned a {:?} credential without {field}",
      cred.r#type
    )));
  }
  cred.to_git2_cred()
}

/// The field a credential of its type cannot be built without, if it is missing.
///
/// `to_git2_cred()` unwraps these, and a panic there would abort the process.
fn missing_required_field(cred: &Credential) -> Option<&'static str> {
  match cred.r#type {
    CredentialType::SSHKeyFromPath if cred.private_key_path.is_none() => Some("privateKeyPath"),
    CredentialType::SSHKey if cred.private_key.is_none() => Some("privateKey"),
    CredentialType::Plain if cred.password.is_none() => Some("password"),
    _ => None,
  }
}

/// Call the user's credential function and send its answer back, waiting for it first if it is a
/// promise.
///
/// Runs on the JavaScript thread. Never fails: every outcome, including a thrown value, is sent as an answer.
fn call_credential_fn(env: &Env, user_fn: &FunctionRef<CredentialCallbackArgs, Unknown>, request: CredentialRequest) {
  let CredentialRequest { args, sender } = request;
  match call_catching(env, user_fn, args) {
    Ok(value) => receive_credential(value, sender),
    Err(answer) => {
      let _ = sender.send(Err(answer));
    }
  }
  // Converting the returned credential runs user code (getters) that may throw; the answer already
  // carries the failure.
  clear_pending_exception(env.raw());
}

fn call_catching<'env>(
  env: &'env Env,
  user_fn: &FunctionRef<CredentialCallbackArgs, Unknown>,
  args: CredentialCallbackArgs,
) -> std::result::Result<Unknown<'env>, String> {
  let failed = |e: Error| format!("failed to call the credential callback: {}", e.reason);
  let func = user_fn.borrow_back(env).map_err(failed)?;
  let raw_env = env.raw();
  let raw_args = unsafe { CredentialCallbackArgs::to_napi_value(raw_env, args) }.map_err(failed)?;
  let mut this = std::ptr::null_mut();
  let mut ret = std::ptr::null_mut();
  let status = unsafe {
    napi::sys::napi_get_undefined(raw_env, &mut this);
    napi::sys::napi_call_function(raw_env, this, func.raw(), 1, [raw_args].as_ptr(), &mut ret)
  };
  if status == napi::sys::Status::napi_pending_exception {
    let mut exception = std::ptr::null_mut();
    unsafe { napi::sys::napi_get_and_clear_last_exception(raw_env, &mut exception) };
    let thrown = unsafe { Unknown::from_napi_value(raw_env, exception) }.map_err(failed)?;
    return Err(format!("credential callback threw: {}", describe_js_value(thrown)));
  }
  if status != napi::sys::Status::napi_ok {
    return Err(failed(Error::from_status(Status::from(status))));
  }
  unsafe { Unknown::from_napi_value(raw_env, ret) }.map_err(failed)
}

/// Turn the callback's return value into an answer, waiting for it first if it is a promise.
///
/// Runs on the JavaScript thread.
fn receive_credential(value: Unknown, sender: mpsc::Sender<CredentialAnswer>) {
  if !matches!(value.is_promise(), Ok(true)) {
    let _ = sender.send(to_credential_answer(value));
    return;
  }
  let promise = PromiseRaw::<Unknown>::new(value.value().env, value.raw());
  let on_fulfilled = sender.clone();
  let on_rejected = sender.clone();
  let chained = promise
    .then(move |ctx| {
      let _ = on_fulfilled.send(to_credential_answer(ctx.value));
      Ok(())
    })
    .and_then(|promise| {
      promise.catch(move |ctx: CallbackContext<Unknown>| {
        let _ = on_rejected.send(Err(format!(
          "credential callback rejected: {}",
          describe_js_value(ctx.value)
        )));
        Ok(())
      })
    });
  if let Err(e) = chained {
    let _ = sender.send(Err(format!("failed to wait for the credential callback: {}", e.reason)));
  }
}

fn to_credential_answer(value: Unknown) -> CredentialAnswer {
  match value.get_type() {
    Ok(ValueType::Null | ValueType::Undefined) => Ok(None),
    _ => unsafe { Credential::from_napi_value(value.value().env, value.raw()) }
      .map(Some)
      .map_err(|e| format!("credential callback returned an invalid credential: {}", e.reason)),
  }
}

fn describe_js_value(value: Unknown) -> String {
  let raw_env = value.value().env;
  // Coercion runs user code (`toString()`) and throws for values such as symbols.
  value
    .coerce_to_string()
    .and_then(|x| x.into_utf8())
    .and_then(|x| x.into_owned())
    .unwrap_or_else(|_| {
      clear_pending_exception(raw_env);
      "unknown error".to_string()
    })
}

/// Drop an exception left by a failed conversion, so it does not escape to the threadsafe function
/// dispatcher, which treats it as fatal.
fn clear_pending_exception(raw_env: napi::sys::napi_env) {
  let mut pending = false;
  unsafe { napi::sys::napi_is_exception_pending(raw_env, &mut pending) };
  if pending {
    let mut exception = std::ptr::null_mut();
    unsafe { napi::sys::napi_get_and_clear_last_exception(raw_env, &mut exception) };
  }
}

fn credential_error(message: impl AsRef<str>) -> git2::Error {
  git2::Error::new(git2::ErrorCode::Auth, git2::ErrorClass::Callback, message.as_ref())
}

#[napi(object)]
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
#[derive(Copy, Clone)]
/// - `Unspecified` : Use the setting from the configuration.
/// - `On` : Force pruning on.
/// - `Off` : Force pruning off
pub enum FetchPrune {
  Unspecified,
  On,
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
#[derive(Copy, Clone)]
/// - `Unspecified` : Use the setting from the remote's configuration
/// - `Auto` : Ask the server for tags pointing to objects we're already downloading
/// - `None` : Don't ask for any tags beyond the refspecs
/// - `All` : Ask for all the tags
pub enum AutotagOption {
  Unspecified,
  Auto,
  None,
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
#[derive(Copy, Clone)]
/// - `None` : Do not follow any off-site redirects at any stage of the fetch or push.
/// - `Initial` : Allow off-site redirects only upon the initial request. This is the default.
/// - `All` : Allow redirects at any stage in the fetch or push.
pub enum RemoteRedirect {
  None,
  Initial,
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
pub struct RemoteTransferProgress {
  pub total_objects: u32,
  pub indexed_objects: u32,
  pub received_objects: u32,
  pub local_objects: u32,
  pub total_deltas: u32,
  pub indexed_deltas: u32,
  pub received_bytes: u32,
}

impl From<git2::Progress<'_>> for RemoteTransferProgress {
  fn from(progress: git2::Progress<'_>) -> Self {
    Self {
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

#[napi(string_enum = "snake_case")]
pub enum PackBuilderStage {
  AddingObjects,
  Deltafication,
}

impl From<git2::PackBuilderStage> for PackBuilderStage {
  fn from(stage: git2::PackBuilderStage) -> Self {
    match stage {
      git2::PackBuilderStage::AddingObjects => PackBuilderStage::AddingObjects,
      git2::PackBuilderStage::Deltafication => PackBuilderStage::Deltafication,
    }
  }
}

#[napi(object)]
pub struct PushUpdate {
  /// The source name of the reference, or `null` if it is not valid UTF-8.
  pub src_refname: Option<String>,
  /// The name of the reference to update on the server, or `null` if it is not valid UTF-8.
  pub dst_refname: Option<String>,
  /// The current target oid of the reference.
  pub src: String,
  /// The new target oid for the reference.
  pub dst: String,
}

impl From<&git2::PushUpdate<'_>> for PushUpdate {
  fn from(update: &git2::PushUpdate<'_>) -> Self {
    let src_refname = std::str::from_utf8(update.src_refname_bytes())
      .ok()
      .map(|x| x.to_string());
    let dst_refname = std::str::from_utf8(update.dst_refname_bytes())
      .ok()
      .map(|x| x.to_string());
    Self {
      src_refname,
      dst_refname,
      src: update.src().to_string(),
      dst: update.dst().to_string(),
    }
  }
}

pub type TransferProgress = JsCallback<RemoteTransferProgress>;
pub type SidebandProgress = JsCallback<Uint8Array>;
pub type UpdateTips = JsCallback<FnArgs<(String, String, String)>>;
pub type PushUpdateReference = JsCallback<FnArgs<(String, Option<String>)>>;
pub type PushTransferProgress = JsCallback<FnArgs<(u32, u32, u32)>>;
pub type PackProgress = JsCallback<FnArgs<(PackBuilderStage, u32, u32)>>;
pub type PushNegotiation = JsCallback<Vec<PushUpdate>>;

#[napi(object, object_to_js = false)]
pub struct RemoteCallbacks {
  /// Called with transfer progress during fetch.
  #[napi(ts_type = "(data: RemoteTransferProgress) => void")]
  pub transfer_progress: Option<TransferProgress>,
  /// Textual progress from the remote.
  ///
  /// Text sent over the progress side-band will be passed to this function
  /// (this is the 'counting objects' output).
  #[napi(ts_type = "(data: Uint8Array) => void")]
  pub sideband_progress: Option<SidebandProgress>,
  /// Each time a reference is updated locally, the callback will be called
  /// with information about it.
  #[napi(ts_type = "(refname: string, oldId: string, newId: string) => void")]
  pub update_tips: Option<UpdateTips>,
  // TODO: certificate_check
  /// Set a callback to get invoked for each updated reference on a push.
  ///
  /// The first argument to the callback is the name of the reference and the
  /// second is a status message sent by the server. If the status is not `null`
  /// then the push was rejected.
  #[napi(ts_type = "(refname: string, status: string | null) => void")]
  pub push_update_reference: Option<PushUpdateReference>,
  /// The callback through which progress of push transfer is monitored
  #[napi(ts_type = "(current: number, total: number, bytes: number) => void")]
  pub push_transfer_progress: Option<PushTransferProgress>,
  /// Function to call with progress information during pack building.
  ///
  /// Be aware that this is called inline with pack building operations,
  /// so performance may be affected.
  #[napi(ts_type = "(stage: PackBuilderStage, current: number, total: number) => void")]
  pub pack_progress: Option<PackProgress>,
  /// The callback is called once between the negotiation step and the upload.
  ///
  /// The argument to the callback is a slice containing the updates which
  /// will be sent as commands to the destination.
  #[napi(ts_type = "(update: PushUpdate[]) => void")]
  pub push_negotiation: Option<PushNegotiation>,
}

#[napi(object, object_to_js = false)]
pub struct FetchOptions {
  /// Credential to authenticate with, or a function that returns one.
  ///
  /// A function is called only when the remote asks for authentication, with the remote URL, the
  /// username in the URL and the credential types the remote accepts. It may return the credential
  /// or a promise for it. It is called again whenever the remote rejects the credential; return
  /// `null`/`undefined` or throw to give up, which fails the operation with that reason. After
  /// 10 calls in one operation the operation fails.
  ///
  /// The operation waits for the function, so a promise that never settles never finishes it.
  /// The wait occupies a libuv threadpool thread, so do not make the function wait for other
  /// threadpool work (such as `fs.promises`) when the pool may be exhausted by waiting operations.
  #[napi(
    ts_type = "Credential | ((args: CredentialCallbackArgs) => Credential | null | undefined | Promise<Credential | null | undefined>)"
  )]
  pub credential: Option<CredentialOption>,
  pub callbacks: Option<RemoteCallbacks>,
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

trait ApplyRemoteCallbacks {
  fn apply(&mut self, cbs: &RemoteCallbacks) -> &mut Self;
}

impl<'a> ApplyRemoteCallbacks for git2::RemoteCallbacks<'a> {
  fn apply(&mut self, cbs: &RemoteCallbacks) -> &mut Self {
    if let Some(cb) = &cbs.transfer_progress {
      self.transfer_progress({
        let cb = cb.clone();
        move |progress| {
          let _ = cb.invoke(RemoteTransferProgress::from(progress));
          true
        }
      });
    }
    if let Some(cb) = &cbs.sideband_progress {
      self.sideband_progress({
        let cb = cb.clone();
        move |data| {
          let _ = cb.invoke(Uint8Array::from(data));
          true
        }
      });
    }
    if let Some(cb) = &cbs.update_tips {
      self.update_tips({
        let cb = cb.clone();
        move |refname, old_oid, new_oid| {
          let refname = refname.to_string();
          let old_oid = old_oid.to_string();
          let new_oid = new_oid.to_string();
          let _ = cb.invoke((refname, old_oid, new_oid).into());
          true
        }
      });
    }
    if let Some(cb) = &cbs.push_update_reference {
      self.push_update_reference({
        let cb = cb.clone();
        move |refname, status| {
          let _ = cb.invoke((refname.to_string(), status.map(|x| x.to_string())).into());
          Ok(())
        }
      });
    }
    if let Some(cb) = &cbs.push_transfer_progress {
      let cb = cb.clone();
      self.push_transfer_progress(move |current, total, bytes| {
        let _ = cb.invoke((current as u32, total as u32, bytes as u32).into());
      });
    }
    if let Some(cb) = &cbs.pack_progress {
      let cb = cb.clone();
      self.pack_progress(move |stage, current, total| {
        let _ = cb.invoke((stage.into(), current as u32, total as u32).into());
      });
    }
    if let Some(cb) = &cbs.push_negotiation {
      let cb = cb.clone();
      self.push_negotiation(move |update| {
        let updates = update.iter().map(PushUpdate::from).collect::<Vec<_>>();
        let _ = cb.invoke(updates);
        Ok(())
      });
    }
    self
  }
}

impl<'a> FetchOptions {
  pub(crate) fn to_git2_fetch_options(&'a self) -> git2::FetchOptions<'a> {
    let mut fetch = git2::FetchOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    if let Some(cred) = &self.credential {
      cred.install(&mut callbacks);
    }
    if let Some(cbs) = &self.callbacks {
      callbacks.apply(cbs);
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

#[napi(object, object_to_js = false)]
/// Options to control the behavior of a git push.
pub struct PushOptions {
  /// Credential to authenticate with, or a function that returns one.
  ///
  /// A function is called only when the remote asks for authentication, with the remote URL, the
  /// username in the URL and the credential types the remote accepts. It may return the credential
  /// or a promise for it. It is called again whenever the remote rejects the credential; return
  /// `null`/`undefined` or throw to give up, which fails the operation with that reason. After
  /// 10 calls in one operation the operation fails.
  ///
  /// The operation waits for the function, so a promise that never settles never finishes it.
  /// The wait occupies a libuv threadpool thread, so do not make the function wait for other
  /// threadpool work (such as `fs.promises`) when the pool may be exhausted by waiting operations.
  #[napi(
    ts_type = "Credential | ((args: CredentialCallbackArgs) => Credential | null | undefined | Promise<Credential | null | undefined>)"
  )]
  pub credential: Option<CredentialOption>,
  pub callbacks: Option<RemoteCallbacks>,
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
      cred.install(&mut callbacks);
    }
    if let Some(cbs) = &self.callbacks {
      callbacks.apply(cbs);
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

#[napi(object, object_to_js = false)]
pub struct FetchRemoteOptions {
  /// Options which can be specified to various fetch operations.
  pub fetch: Option<FetchOptions>,
  pub reflog_msg: Option<String>,
}

#[napi(object, object_to_js = false)]
pub struct PruneOptions {
  /// Credential to authenticate with, or a function that returns one.
  ///
  /// Pruning compares against the references from the last connection to the remote and does not
  /// connect again, so this is currently never used.
  #[napi(
    ts_type = "Credential | ((args: CredentialCallbackArgs) => Credential | null | undefined | Promise<Credential | null | undefined>)"
  )]
  pub credential: Option<CredentialOption>,
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
        cred.install(&mut callbacks);
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
  type JsValue = String;

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

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.to_owned())
  }
}

#[napi]
/// A class representing a [remote][1] of a git repository.
///
/// [1]: https://git-scm.com/book/en/Git-Basics-Working-with-Remotes
pub struct Remote {
  pub(crate) inner: SharedReference<Repository, git2::Remote<'static>>,
}

#[napi]
impl Remote {
  #[napi]
  /// Get the remote's name.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   name(): string | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if this remote has not yet been named.
  /// @throws Throws error if the name is not valid utf-8.
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
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   url(): string;
  /// }
  /// ```
  ///
  /// @throws Throws error if the URL is not valid utf-8.
  pub fn url(&self) -> crate::Result<String> {
    let url = std::str::from_utf8(self.inner.url_bytes())?.to_string();
    Ok(url)
  }

  #[napi]
  /// Get the remote's URL.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   pushurl(): string | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if push url not exists.
  /// @throws Throws error if the URL is not valid utf-8.
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
  /// Filter refspec if has not valid `src` or `dst` with utf-8.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   refspecs(): Refspec[];
  /// }
  /// ```
  ///
  /// @returns List all refspecs.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  /// const remote = repo.getRemote('origin');
  ///
  /// // Retrieving the Refspecs configured for this remote
  /// const refspecs = remote.refspecs();
  /// console.log(refspecs[0]);
  /// // For the "+refs/heads/*:refs/remotes/origin/*" Refspec
  /// // {
  /// //   "direction": "Fetch",
  /// //   "src": "refs/heads/*",
  /// //   "dst": "refs/remotes/origin/*",
  /// //   "force": true
  /// // }
  /// ```
  pub fn refspecs(&self) -> Vec<Refspec> {
    self
      .inner
      .refspecs()
      .filter_map(|x| Refspec::try_from(x).ok())
      .collect::<Vec<_>>()
  }

  #[napi]
  /// Download new data and update tips.
  ///
  /// Convenience function to connect to a remote, download the data, disconnect and update the remote-tracking branches.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   fetch(
  ///     refspecs: string[],
  ///     options?: FetchRemoteOptions | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {string[]} refspecs - Refspecs to fetch from remote.
  /// @param {FetchRemoteOptions} [options] - Options for fetch remote.
  /// @param {AbortSignal} [signal] Abort signal.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  /// const remote = repo.getRemote('origin');
  ///
  /// // Fetching data from the "main" branch
  /// await remote.fetch(['main']);
  ///
  /// // Providing an empty array fetches data using the default Refspec configured for the remote
  /// await remote.fetch([]);
  /// ```
  ///
  /// Pick a credential when the remote asks for one.
  ///
  /// ```ts
  /// await remote.fetch(['main'], {
  ///   fetch: {
  ///     credential: ({ url, usernameFromUrl, allowedTypes }) => {
  ///       if (allowedTypes.includes('SSHKeyFromAgent')) {
  ///         return { type: 'SSHKeyFromAgent', username: usernameFromUrl ?? 'git' };
  ///       }
  ///       // Return `null` to give up.
  ///       return { type: 'Plain', password: tokenFor(new URL(url).host) };
  ///     },
  ///   },
  /// });
  /// ```
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
  /// Perform a push.
  ///
  /// Perform all the steps for a push.
  /// If no refspecs are passed, then the configured refspecs will be used.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   push(
  ///     refspecs: string[],
  ///     options?: PushOptions | null | undefined,
  ///     signal?: AbortSignal | null | undefined,
  ///   ): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {string[]} refspecs - Refspecs to push to remote.
  /// @param {FetchRemoteOptions} [options] - Options for push remote.
  /// @param {AbortSignal} [signal] Abort signal.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  /// const remote = repo.getRemote('origin');
  ///
  /// // Push the local "main" branch to the remote "other" branch
  /// await remote.push(['refs/heads/main:refs/heads/other']);
  ///
  /// // Push with credential.
  /// await remote.push(['refs/heads/main:refs/heads/other'], {
  ///   credential: {
  ///     type: 'Plain',
  ///     password: '<personal access token>',
  ///   },
  /// });
  /// ```
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
  /// Prune tracking refs that are no longer present on remote.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   prune(options?: PruneOptions | null | undefined, signal?: AbortSignal | null | undefined): Promise<void>;
  /// }
  /// ```
  ///
  /// @param {PruneOptions} [options] - Options for prune remote.
  /// @param {AbortSignal} [signal] Abort signal.
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
  ///
  /// The `fetch` operation from the remote is also performed.
  ///
  /// @category Remote/Methods
  /// @signature
  /// ```ts
  /// class Remote {
  ///   defaultBranch(signal?: AbortSignal | null | undefined): Promise<string>;
  /// }
  /// ```
  ///
  /// @param {AbortSignal} [signal] Abort signal.
  /// @returns Default branch name.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  /// const remote = repo.getRemote('origin');
  ///
  /// const branch = await remote.defaultBranch();
  /// console.log(branch); // "refs/heads/main"
  /// ```
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
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   remoteNames(): string[];
  /// }
  /// ```
  ///
  /// @returns All remote names for this repository.
  ///
  /// @example
  /// ```ts
  /// import { openRepository } from 'es-git';
  ///
  /// const repo = await openRepository('/path/to/repo');
  /// console.log(repo.remoteNames()); // ["origin"]
  /// ```
  pub fn remote_names(&self) -> crate::Result<Vec<String>> {
    let remotes = self
      .inner
      .remotes()
      .map(|names| names.into_iter().flatten().map(|x| x.to_owned()).collect::<Vec<_>>())?;
    Ok(remotes)
  }

  #[napi]
  /// Get remote from repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   getRemote(name: string): Remote;
  /// }
  /// ```
  ///
  /// @returns Remote instance.
  /// @throws Throws error if remote does not exist.
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
  /// Find remote from repository.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   findRemote(name: string): Remote | null;
  /// }
  /// ```
  ///
  /// @returns Returns `null` if remote does not exist.
  pub fn find_remote(&self, this: Reference<Repository>, env: Env, name: String) -> Option<Remote> {
    self.get_remote(this, env, name).ok()
  }

  #[napi]
  /// Add a remote with the default fetch refspec to the repository’s configuration.
  ///
  /// @category Repository/Methods
  /// @signature
  /// ```ts
  /// class Repository {
  ///   createRemote(name: string, url: string, options?: CreateRemoteOptions | null | undefined): Remote;
  /// }
  /// ```
  ///
  /// @param {string} name - The name of the remote.
  /// @param {string} url - Remote url.
  /// @param {CreateRemoteOptions} [options] - Options for creating remote.
  /// @returns Created remote.
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
