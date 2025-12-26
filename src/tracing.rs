use crate::js::{JsCallback, JsCallbackExt};
use napi::bindgen_prelude::FnArgs;
use napi_derive::napi;
use std::sync::{Mutex, OnceLock};

/// Available tracing levels.  When tracing is set to a particular level,
/// callers will be provided tracing at the given level and all lower levels.
#[napi(string_enum)]
pub enum TraceLevel {
  None,
  Fatal,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

impl From<TraceLevel> for git2::TraceLevel {
  fn from(value: TraceLevel) -> Self {
    match value {
      TraceLevel::None => git2::TraceLevel::None,
      TraceLevel::Fatal => git2::TraceLevel::Fatal,
      TraceLevel::Error => git2::TraceLevel::Error,
      TraceLevel::Warn => git2::TraceLevel::Warn,
      TraceLevel::Info => git2::TraceLevel::Info,
      TraceLevel::Debug => git2::TraceLevel::Debug,
      TraceLevel::Trace => git2::TraceLevel::Trace,
    }
  }
}

impl From<git2::TraceLevel> for TraceLevel {
  fn from(value: git2::TraceLevel) -> Self {
    match value {
      git2::TraceLevel::None => TraceLevel::None,
      git2::TraceLevel::Fatal => TraceLevel::Fatal,
      git2::TraceLevel::Error => TraceLevel::Error,
      git2::TraceLevel::Warn => TraceLevel::Warn,
      git2::TraceLevel::Info => TraceLevel::Info,
      git2::TraceLevel::Debug => TraceLevel::Debug,
      git2::TraceLevel::Trace => TraceLevel::Trace,
    }
  }
}

pub type TracingCallback = JsCallback<FnArgs<(TraceLevel, String)>>;

#[allow(clippy::type_complexity)]
static TRACING_CALLBACK: OnceLock<Mutex<Option<TracingCallback>>> = OnceLock::new();

fn tracing_callback(level: git2::TraceLevel, msg: &[u8]) {
  let Some(lock) = TRACING_CALLBACK.get() else { return };
  let Ok(guard) = lock.lock() else { return };
  let Some(callback) = guard.as_ref() else { return };
  if let Ok(message) = std::str::from_utf8(msg) {
    let _ = callback.invoke((level.into(), message.to_string()).into());
  }
}

#[napi]
/// Set the global subscriber called when libgit2 produces a tracing message.
///
/// @category Tracing
/// @signature
/// ```ts
/// function traceSet(
///   level: TraceLevel,
///   callback: (level: TraceLevel, message: string) => void,
/// ): void;
/// ```
///
/// @param {TraceLevel} level - Level to set tracing to
/// @param {(level: TraceLevel, message: string) => void} callback - Callback to call with trace data
pub fn trace_set(
  level: TraceLevel,
  #[napi(ts_arg_type = "(level: TraceLevel, message: string) => void")] callback: TracingCallback,
) -> crate::Result<()> {
  let global = TRACING_CALLBACK.get_or_init(|| Mutex::new(None));
  *global.lock().unwrap() = Some(callback);
  git2::trace_set(level.into(), tracing_callback)?;
  Ok(())
}

#[napi]
/// Clear the global subscriber
///
/// @category Tracing
/// @signature
/// ```ts
/// function traceClear(): void;
/// ```
pub fn trace_clear() {
  if let Some(global) = TRACING_CALLBACK.get() {
    *global.lock().unwrap() = None;
  }
}
