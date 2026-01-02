use bitflags::Flags;
use std::path::Path;

pub(crate) fn path_to_string(p: &Path) -> String {
  #[cfg(unix)]
  {
    p.to_string_lossy().to_string()
  }
  #[cfg(windows)]
  {
    use std::os::windows::ffi::OsStrExt;
    let path_buf = p.as_os_str().encode_wide().collect::<Vec<u16>>();
    let str = String::from_utf16_lossy(path_buf.as_slice()).to_string();
    str
  }
}

pub(crate) fn bitflags_contain<T: Flags>(source: T, target: T) -> bool {
  source.contains(target)
}

/// Macro to create a `PromiseRaw<T>` from a closure.
///
/// - The closure should return `crate::Result<T>`
/// - `Ok(value)` becomes `PromiseRaw::resolve`
/// - `Err(error)` becomes `PromiseRaw::reject`
/// - Errors from `?` operator are automatically converted to rejected promises
///
/// # Example
/// ```rust,ignore
/// use napi::bindgen_prelude::PromiseRaw;
///
/// fn my_async_fn(env: Env) -> PromiseRaw<'_, MyType> {
///   napi_promise!(&env, || {
///     let value = some_operation()?;  // errors are caught and rejected
///     Ok(MyType { value })
///   })
/// }
/// ```
#[macro_export]
macro_rules! napi_promise {
  ($env:expr, || $($body:tt)*) => {{
    match (|| -> $crate::Result<_> { $($body)* })() {
      Ok(value) => PromiseRaw::resolve($env, value).expect("napi_promise: failed to create resolved promise"),
      Err(error) => PromiseRaw::reject($env, error.to_string()).expect("napi_promise: failed to create rejected promise")
    }
  }};
}
